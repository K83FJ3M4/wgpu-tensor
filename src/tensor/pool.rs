use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crossbeam_channel::{Receiver, Sender, unbounded};
use wgpu::BufferSize;
use crate::tensor::{TensorDomain, TensorInner};
use crate::{IntoShape, Tensor, TensorEncoder, TensorError};

pub(crate) struct TensorPool {
    sender: Sender<TensorInner<'static>>,
    receiver: Receiver<TensorInner<'static>>,
    tensors: Mutex<HashMap<BufferSize, Vec<TensorInner<'static>>>>
}

impl TensorPool {
    pub(crate) fn new() -> TensorPool {
        let (sender, receiver) = unbounded();
        TensorPool {
            tensors: Mutex::new(HashMap::new()),
            receiver,
            sender
        }
    } 
}

impl<'scope> TensorEncoder<'scope> {
    pub(crate) fn temp(
        &mut self,
        shape: impl IntoShape,
    ) -> Result<Tensor<'scope>, TensorError> {
        match self.tensors.tensors.lock() {
            Ok(mut tensors) => {
                while let Ok(tensor) = self.tensors.receiver.try_recv() {
                    let size = BufferSize::new(tensor.buffer.size());
                    let Some(size) = size else { continue };
                    tensors.entry(size)
                        .or_default()
                        .push(tensor)
                }
            },
            Err(err) => {
                let mut map = err.into_inner();
                *map = HashMap::new();
                self.tensors.tensors.clear_poison();
            }
        }

        let shape = shape.shape();
        let bucket = Tensor::bucket_size(shape)?;
 
        if let Ok(mut tensors) = self.tensors.tensors.lock() {
            if let Some(mut tensor) = tensors.get_mut(&bucket)
                .map(Vec::pop).flatten() {
                tensor.domain = TensorDomain::Temporary(&self.tensors.sender);
                tensor.shape = Arc::new(shape);
                return Ok(super::Tensor(Arc::new(tensor))); 
            }
        } 

        let tensor = Tensor::create(
            self.encoder.pipelines(),
            shape, bucket,
            TensorDomain::Temporary(&self.tensors.sender)
        );
        Ok(super::Tensor(Arc::new(tensor)))
    }
}