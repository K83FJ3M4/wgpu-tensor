use wgpu::WriteOnly;

use crate::staging::allocator::StagingChunk;
use crate::{Tensor, TensorEncoder};
pub(crate) use allocator::{StagingAllocator, StagingAllocatorPool};

mod allocator;

pub trait TensorReader: Send + 'static {
    fn read(&mut self, data: &[u8]);
    fn error(&mut self) {}
}

impl<T: Iterator<Item = u8>> TensorWriter for T {
    fn write(&mut self, chunk: WriteOnly<[u8]>) {
        let len = chunk.len();
        chunk.write_iter(self.take(len));
    }
}

pub trait TensorWriter {
    fn write(&mut self, chunk: WriteOnly<[u8]>);
}

impl<'a> TensorEncoder<'a> {
    pub fn write(&mut self, tensor: &Tensor, mut writer: impl TensorWriter) {
        let mut size = tensor.data_size();
        let mut length = tensor.buffer.size();
        let mut offset = 0;

        while length != 0 {
            let mut chunk = self.write_allocator.chunk(
                length,
                self.encoders.command()
            );

            let data = chunk.data();
            let data_len = data.len().min(size);
            writer.write(chunk.data().into_slice(..data_len));

            let slice = chunk.slice();
            self.encoders.command().copy_buffer_to_buffer(
                slice.buffer(), slice.offset(),
                &tensor.buffer, offset, slice.size()
            );

            offset += slice.size();
            length -= slice.size();
            size -= data_len;
        }
    }

    pub fn read(&mut self, tensor: &Tensor, mut reader: impl TensorReader) {
        let mut offset = 0;
        let mut length = tensor.buffer.size();
        let mut chunks = Vec::new();

        while length != 0 {
            let chunk = self.read_allocator.chunk(
                length,
                self.encoders.command()
            );

            let slice = chunk.slice();
            self.encoders.command().copy_buffer_to_buffer(
                &tensor.buffer, offset,
                slice.buffer(), slice.offset(),
                slice.size()
            );
            
            offset += slice.size();
            length -= slice.size();
            chunks.push(chunk.unmap());
        }

        let mut size = tensor.data_size();
        self.encoders.command().on_submitted_work_done(move || {
            if !chunks.iter().all(StagingChunk::mapped) { reader.error() }
            for slice in chunks.iter().map(StagingChunk::slice) { 
                if let Ok(range) = slice.get_mapped_range() {
                    reader.read(&range[..range.len().min(size)]);
                    size -= size.saturating_sub(range.len());
                } else {
                    reader.error();
                    break;
                }
            }
        });
    }
}