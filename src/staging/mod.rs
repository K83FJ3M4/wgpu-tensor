use crate::staging::allocator::StagingChunk;
use crate::{Tensor, TensorEncoder, TensorError};
pub(crate) use allocator::{StagingAllocator, StagingAllocatorPool};
pub use reader::{TensorReceiver, TensorChunks, TensorReader, F32Iter};
pub(crate) use encoder::{Encoder, EncoderPool};
pub use writer::TensorWriter;

mod allocator;
mod writer;
mod reader;
mod encoder;

impl<'scope> TensorEncoder<'scope> {
    pub fn write(
        &mut self,
        tensor: &Tensor<'static>,
        mut writer: impl TensorWriter,
    ) -> Result<(), TensorError> {
        self.validate_write(tensor)?;

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
            writer.write(chunk.data().into_slice(..data_len))?;

            let slice = chunk.slice();
            self.encoder.command().copy_buffer_to_buffer(
                slice.buffer(), slice.offset(),
                &tensor.data_buffer(), offset, slice.size()
            );

            offset += slice.size();
            length -= slice.size();
            size -= data_len;
        }

        writer.finish()
    }

    pub fn read(&mut self, tensor: &Tensor) -> TensorReceiver {
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

        let size = Tensor::data_size(tensor.shape()).unwrap() as usize;
        TensorReceiver {
            views: None,
            chunks,
            size
        } 
    }
}
