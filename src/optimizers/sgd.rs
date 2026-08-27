use crate::{Tensor, TensorContext, TensorEncoder, TensorError};
use crate::optimizers::{Optimizer, OptimizerConfig};

#[derive(Clone, Copy)]
pub struct SGDConfig {
    pub learning_rate: f32
}

pub struct SGDOptimizer {
    learning_rate: f32
}

impl OptimizerConfig for SGDConfig {
    type Optimizer = SGDOptimizer;

    fn build(
        self,
        _: &mut TensorContext
    ) -> Self::Optimizer {
        SGDOptimizer {
            learning_rate: self.learning_rate
        } 
    }
}

impl Optimizer for SGDOptimizer {
    fn optimize<'scope>(
        &mut self,
        encoder: &mut TensorEncoder<'scope>,
        gradients: &Tensor<'scope>,
        weights: &Tensor<'scope>
    ) -> Result<(), TensorError> {
        encoder.optimize(gradients, weights, self.learning_rate)
    }
}
