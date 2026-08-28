use crate::layers::Layer;
use crate::optimizers::OptimizerConfig;
use crate::{Tensor, TensorContext, TensorEncoder, TensorError};

pub struct Dense {
    weights: Tensor<'static>,
    bias: Tensor<'static>,
}

impl Dense {
    pub fn new(
        context: &TensorContext,
        input_size: u32,
        output_size: u32,
        optimizer: impl OptimizerConfig + Clone,
    ) -> Result<Dense, TensorError> {
        let weights = Tensor::new_trainable(
            context,
            optimizer.clone(),
            (output_size, input_size),
        )?;
        let bias = Tensor::new_trainable(
            context,
            optimizer,
            (output_size, 1),
        )?;

        Ok(Dense { weights, bias })
    }
}

impl Layer for Dense {
    fn init(
        &self,
        encoder: &mut TensorEncoder,
    ) -> Result<(), TensorError> {
        let shape = self.weights.shape();
        encoder.xavier_uniform(&self.weights, shape[1], shape[0])?;
        encoder.fill(&self.bias, 0.0)
    }

    fn forward<'scope>(
        &self,
        encoder: &mut TensorEncoder<'scope>,
        input: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {
        let output = encoder.matmul(input, &self.weights)?;
        encoder.add(&output, &self.bias)
    }
}
