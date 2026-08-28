use std::slice::Iter;
use bytemuck::cast_slice;
use wgpu::BufferView;

use crate::TensorError;
use crate::staging::StagingChunk;

pub trait TensorReader<'a>: Sized {
    fn new(chunks: TensorChunks<'a>) -> Result<Self, TensorError>;
}

pub struct TensorReceiver {
    pub(super) views: Option<Vec<BufferView>>,
    pub(super) chunks: Vec<StagingChunk<BufferView>>,
    pub(super) size: usize
}

pub struct TensorChunks<'a> {
    views: Iter<'a, BufferView>,
    size: usize
}

pub struct F32Iter<'a> {
    chunks: TensorChunks<'a>,
    chunk: Option<&'a [f32]>
}

impl TensorReceiver {
    pub fn try_recv<'a, T: TensorReader<'a>>(&'a mut self) -> Result<T, TensorError> {
        if !self.chunks.iter().all(StagingChunk::mapped) {
            return Err(TensorError::StagingBufferNotMapped)
        } 

        let views = match self.views.take() {
            Some(views) => views,
            None => {
                let mut views = Vec::new();
                for chunk in self.chunks.iter() {
                    let slice = chunk.slice();
                    let view = slice.get_mapped_range()
                        .map_err(|_| TensorError::StagingBufferMapFailed)?;
                    views.push(view);
                }
                views
            }
        };

        let views = self.views.insert(views);

        Ok(T::new(TensorChunks {
            size: self.size,
            views: views.iter()
        })?)
    }
}

impl<'a> Iterator for TensorChunks<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let view = self.views.next()?;
        let length = view.len().min(self.size);
        let view = &view[..length];
        self.size -= view.len();
        Some(view)
    }
}

impl<'a> TensorReader<'a> for F32Iter<'a> {
    fn new(chunks: TensorChunks<'a>) -> Result<Self, TensorError> {
        Ok(Self {
            chunks,
            chunk: None
        })
    }
}

impl<'a> Iterator for F32Iter<'a> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((item, remainder)) = self.chunk
                .map(<[f32]>::split_first)
                .flatten() {
                self.chunk = Some(remainder);
                return Some(*item)
            } else {
                let chunk = self.chunks.next()?;
                self.chunk = Some(cast_slice(chunk));
            }
        } 
    }
}