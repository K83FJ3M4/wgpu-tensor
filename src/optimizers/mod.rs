use crate::{Tensor, TensorContext, TensorEncoder};

pub trait OptimizerConfig {
    type Optimizer: Optimizer;

    fn build(
        self,
        context: &mut TensorContext
    ) -> Self::Optimizer;
}

pub trait Optimizer: 'static + Send {
    fn update(
        &mut self,
        encoder: &mut TensorEncoder,
        gradients: &Tensor,
        weights: &Tensor
    );
}