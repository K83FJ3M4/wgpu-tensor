use std::marker::PhantomData;
use std::ops::Range;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crossbeam_channel::{Receiver, Sender, unbounded};
use wgpu::{Buffer, BufferDescriptor, BufferSlice, BufferUsages, BufferView, BufferViewMut, CommandEncoder, Device, MAP_ALIGNMENT, MapMode, WriteOnly};

const STATUS_UNMAPPED: usize = 0;
const STATUS_MAPPED: usize = 1;
const STATUS_FAILED: usize = 2;

pub(crate) struct StagingAllocatorPool<T: StagingBufferView> {
    receiver: Receiver<Buffer>,
    marker: PhantomData<T>,
    sender: Sender<Buffer>,
    device: Device,
}

pub(crate) struct StagingAllocator<'a, T: StagingBufferView> {
    pool: &'a StagingAllocatorPool<T>,
    buffer: Option<MappedStagingBuffer<T>>,
    offset: u64
}

pub(super) struct MappedStagingChunk<'a, T: StagingBufferView> {
    buffer: Arc<StagingBuffer>,
    view: &'a mut T,
    start: u64,
    end: u64
}

pub(super) struct StagingChunk<T> {
    marker: PhantomData<T>,
    buffer: Arc<StagingBuffer>,
    start: u64,
    end: u64
}

struct MappedStagingBuffer<T: StagingBufferView> {
    inner: Arc<StagingBuffer>,
    view: T
}

struct StagingBuffer {
    status: AtomicUsize,
    sender: Sender<Buffer>,
    buffer: Buffer
}

pub(crate) trait StagingBufferView: Sized {
    type Data<'a> where Self: 'a;
    const BUFFER_USAGES: BufferUsages;
    const MAP_MODE: wgpu::MapMode;
    const BLOCK_SIZE: u64;

    fn new(buffer: &Buffer) -> Option<Self>;
    fn data<'a>(&'a mut self, range: Range<u64>) -> Self::Data<'a>;
}

impl<'a, T: StagingBufferView> StagingAllocator<'a, T> {
    pub(crate) fn new(
        pool: &'a StagingAllocatorPool<T>
    ) -> StagingAllocator<'a, T> {
        StagingAllocator {
            buffer: None,
            offset: 0,
            pool
        }
    }

    pub(super) fn chunk<'b>(
        &'b mut self, size: u64,
        encoder: &mut CommandEncoder,
    ) -> MappedStagingChunk<'b, T> {
        self.offset = self.offset
            .next_multiple_of(MAP_ALIGNMENT)
            .min(T::BLOCK_SIZE);

        let swap_buffer = self.buffer.as_ref().map(|block| {
            block.inner.buffer.size() - self.offset == 0
        }).unwrap_or(true);

        if swap_buffer && let Some(buffer) = self.buffer.take() {
            self.offset = 0;
            drop(buffer.view);
            buffer.inner.buffer.unmap();
            buffer.inner.status.store(
                STATUS_UNMAPPED,
                Ordering::Release
            )
        }

        let buffer = match self.buffer.take() {
            Some(block) if !swap_buffer => block,
            Some(..) | None => self.pool.buffer()
        }; 

        if swap_buffer {
            let inner_buffer = buffer.inner.clone();
            encoder.map_buffer_on_submit(
                &buffer.inner.buffer, T::MAP_MODE, ..,
                move |result| inner_buffer.status.store(
                    if result.is_ok() {
                        STATUS_MAPPED
                    } else {
                        STATUS_FAILED
                    },
                    Ordering::Release
                ) 
            );
        }

        let start = self.offset;
        let length = size.min(T::BLOCK_SIZE - start);
        let end = start + length;
        self.offset = end;
        let buffer = self.buffer.insert(buffer);
        MappedStagingChunk {
            buffer: buffer.inner.clone(),
            view: &mut buffer.view,
            start,
            end,
        }
    }
}

impl<T: StagingBufferView> StagingAllocatorPool<T> {
    pub(crate) fn new(device: Device) -> StagingAllocatorPool<T> {
        let (sender, receiver) = unbounded();
        StagingAllocatorPool {
            marker: PhantomData,
            receiver,
            sender,
            device
        }
    }

    fn buffer(&self) -> MappedStagingBuffer<T> {
        while let Ok(buffer) = self.receiver.try_recv() {
            if let Some(view) = T::new(&buffer) {
                let staging_buffer = StagingBuffer {
                    status: AtomicUsize::new(STATUS_MAPPED),
                    sender: self.sender.clone(),
                    buffer
                };

                return MappedStagingBuffer {
                    inner: Arc::new(staging_buffer),
                    view
                }
            }
        }

        let buffer = self.device.create_buffer(&BufferDescriptor {
            usage: T::BUFFER_USAGES,
            mapped_at_creation: true,
            size: T::BLOCK_SIZE,
            label: None
        });

        let view = T::new(&buffer).unwrap();
        let staging_buffer = StagingBuffer {
            status: AtomicUsize::new(STATUS_MAPPED),
            sender: self.sender.clone(),
            buffer
        };

        MappedStagingBuffer {
            inner: Arc::new(staging_buffer),
            view
        }
    }
}

impl<'a, T: StagingBufferView> MappedStagingChunk<'a, T> {
    pub(super) fn unmap(self) -> StagingChunk<T> {
        StagingChunk {
            marker: PhantomData,
            buffer: self.buffer.clone(),
            start: self.start,
            end: self.end
        }
    }

    pub(super) fn slice<'b>(&'b self) -> BufferSlice<'b> {
        self.buffer.buffer.slice(self.start..self.end)
    }

    pub(super) fn data<'b>(&'b mut self) -> T::Data<'b> {
        T::data(&mut self.view, self.start..self.end)
    }
}

impl<T> StagingChunk<T> {
    pub(super) fn slice<'b>(&'b self) -> BufferSlice<'b> {
        self.buffer.buffer.slice(self.start..self.end)
    }

    pub(super) fn mapped(&self) -> bool {
        self.buffer.status.load(Ordering::Acquire) == STATUS_MAPPED
    }
}


impl StagingBufferView for BufferViewMut {
    type Data<'a> = WriteOnly<'a, [u8]>;

    const BLOCK_SIZE: u64 = 4 << 20;
    const MAP_MODE: MapMode = MapMode::Write;
    const BUFFER_USAGES: BufferUsages
        = BufferUsages::from_bits(
            BufferUsages::COPY_SRC.bits()
            | BufferUsages::MAP_WRITE.bits()
    ).unwrap();

    fn new(buffer: &Buffer) -> Option<Self> {
        buffer.get_mapped_range_mut(..).ok()
    }

    fn data<'a>(&'a mut self, range: Range<u64>) -> Self::Data<'a> {
        self.slice(range.start as usize..range.end as usize)
    }
}

impl StagingBufferView for BufferView {
    type Data<'a> = &'a [u8];

    const BLOCK_SIZE: u64 = 4 << 20;
    const MAP_MODE: MapMode = MapMode::Read;
    const BUFFER_USAGES: BufferUsages
        = BufferUsages::from_bits(
            BufferUsages::COPY_DST.bits()
            | BufferUsages::MAP_READ.bits()
    ).unwrap();

    fn new(buffer: &Buffer) -> Option<Self> {
        buffer.get_mapped_range(..).ok()
    }

    fn data<'a>(&'a mut self, range: Range<u64>) -> Self::Data<'a> {
        &self[range.start as usize..range.end as usize] 
    }
}

impl<'a, T: StagingBufferView> Drop for StagingAllocator<'a, T> {
    fn drop(&mut self) {
        let Some(buffer) = self.buffer.take() else { return };
        drop(buffer.view);
        buffer.inner.buffer.unmap();
        buffer.inner.status.store(
            STATUS_UNMAPPED,
            Ordering::Release
        );
    } 
}

impl Drop for StagingBuffer {
    fn drop(&mut self) {
        if self.status.load(Ordering::Acquire) == STATUS_MAPPED {
            self.sender.send(self.buffer.clone()).ok();
        }
    }
}