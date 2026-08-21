use crate::staging::allocator::StagingChunk;
use crate::{Tensor, TensorEncoder};
pub(crate) use allocator::{StagingAllocator, StagingAllocatorPool};
pub use reader::{TensorReader, PrintTensorReader};
pub(crate) use encoder::{Encoder, EncoderPool};
pub use writer::TensorWriter;

mod allocator;
mod writer;
mod reader;
mod encoder;

impl<'scope> TensorEncoder<'scope> {
    pub fn write(&mut self, tensor: &Tensor, mut writer: impl TensorWriter) {
        let mut size = Tensor::data_size(tensor.shape()).unwrap() as usize;
        let mut length = Tensor::buffer_size(tensor.shape()).unwrap().get();
        let mut offset = 0;

        while length != 0 {
            let mut chunk = self.write_allocator.chunk(
                length,
                self.encoder.command()
            );

            let data = chunk.data();
            let data_len = data.len().min(size);
            writer.write(chunk.data().into_slice(..data_len));

            let slice = chunk.slice();
            self.encoder.command().copy_buffer_to_buffer(
                slice.buffer(), slice.offset(),
                &tensor.data_buffer(), offset, slice.size()
            );

            offset += slice.size();
            length -= slice.size();
            size -= data_len;
        }

        writer.finish();
    }

    pub fn read(&mut self, tensor: &Tensor, mut reader: impl TensorReader) {
        let mut offset = 0;
        let mut length = Tensor::buffer_size(tensor.shape()).unwrap().get();
        let mut chunks = Vec::new();

        while length != 0 {
            let chunk = self.read_allocator.chunk(
                length,
                self.encoder.command()
            );

            let slice = chunk.slice();
            self.encoder.command().copy_buffer_to_buffer(
                &tensor.data_buffer(), offset,
                slice.buffer(), slice.offset(),
                slice.size()
            );
            
            offset += slice.size();
            length -= slice.size();
            chunks.push(chunk.unmap());
        }

        let mut size = Tensor::data_size(tensor.shape()).unwrap() as usize;
        self.encoder.command().on_submitted_work_done(move || {
            if !chunks.iter().all(StagingChunk::mapped) { reader.error() }
            for slice in chunks.iter().map(StagingChunk::slice) { 
                if let Ok(range) = slice.get_mapped_range() {
                    let length = range.len().min(size);
                    reader.read(&range[..length]);
                    size -= length;
                } else {
                    reader.error();
                    break;
                }
            }

            reader.finish();
        });
    }
}