use crate::layers::Layer;
use crate::{Tensor, TensorEncoder, TensorError};

pub struct Dense {
    weights: Tensor<'static>,
    bias: Tensor<'static>,
}

impl Layer for Dense {
    fn forward<'scope>(
        &self,
        encoder: &mut TensorEncoder<'scope>,
        input: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {
        let output = encoder.matmul(input, &self.weights)?;
        encoder.add(&output, &self.bias)
    }
}