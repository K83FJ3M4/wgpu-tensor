use wgpu::WriteOnly;

use crate::TensorError;

pub trait TensorWriter {
    fn write(
        &mut self,
        chunk: WriteOnly<[u8]>,
    ) -> Result<(), TensorError>;

    fn finish(&mut self) -> Result<(), TensorError> {
        Ok(())
    }
}

impl<T: Iterator<Item = f32>> TensorWriter for T {
    fn write(
        &mut self,
        chunk: WriteOnly<[u8]>,
    ) -> Result<(), TensorError> {
        let (values, remainder) = chunk.into_chunks::<{ size_of::<f32>() }>();
        debug_assert!(remainder.is_empty());

        for destination in values {
            let Some(value) = self.next() else {
                return Err(TensorError::InsufficientData)
            };
            destination.write(value.to_ne_bytes());
        }

        Ok(())
    }

    fn finish(&mut self) -> Result<(), TensorError> {
        if self.next().is_some() {
            Err(TensorError::ExcessData)
        } else {
            Ok(())
        }
    }
}