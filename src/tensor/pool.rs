use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};
use wgpu::BufferSize;
use crate::tensor::TensorInner;
use crate::{IntoShape, Tensor, TensorEncoder, TensorError};

pub(crate) struct TensorPool {
    sender: Sender<TensorInner<'static>>,
    receiver: Receiver<TensorInner<'static>>,
    tensors: RefCell<HashMap<BufferSize, Vec<TensorInner<'static>>>>
}

impl TensorPool {
    pub(crate) fn new() -> TensorPool {
        let (sender, receiver) = channel();
        TensorPool {
            tensors: RefCell::new(HashMap::new()),
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
        if let Ok(mut tensors) = self.tensors.tensors.try_borrow_mut() {
            while let Ok(tensor) = self.tensors.receiver.try_recv() {
                let size = BufferSize::new(tensor.buffer.size());
                let Some(size) = size else { continue };
                tensors.entry(size)
                    .or_default()
                    .push(tensor)
            }
        }

        let shape = shape.shape();
        let bucket = Tensor::bucket_size(shape)?;
 
        if let Ok(mut tensors) = self.tensors.tensors.try_borrow_mut() {
            if let Some(mut tensor) = tensors.get_mut(&bucket)
                .map(Vec::pop).flatten() {
                tensor.sender = Some(&self.tensors.sender);
                tensor.shape = shape;
                return Ok(super::Tensor(Arc::new(tensor))); 
            }
        } 

        let mut tensor = Tensor::create(
            self.encoder.pipelines(),
            shape, bucket,
            None
        );
        tensor.sender = Some(&self.tensors.sender);
        Ok(super::Tensor(Arc::new(tensor)))
    }
}