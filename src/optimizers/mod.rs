use std::collections::{HashMap, HashSet};

use crate::tensor::WeakTensor;
use crate::{IntoShape, Tensor, TensorContext, TensorEncoder, TensorError};
pub use sgd::{SGDConfig, SGDOptimizer};

mod sgd;

pub trait OptimizerConfig {
    type Optimizer: Optimizer;

    fn build(
        self,
        context: &mut TensorContext
    ) -> Self::Optimizer;
}

pub trait Optimizer: 'static + Send {
    fn optimize<'scope>(
        &mut self,
        encoder: &mut TensorEncoder<'scope>,
        gradients: &Tensor<'scope>,
        weights: &Tensor<'scope>
    ) -> Result<(), TensorError>;
}

pub(crate) struct AutogradEncoder<'scope> {
    tracked: HashSet<WeakTensor<'scope>>,
    operations: Vec<Box<dyn FnOnce(
        &mut TensorEncoder<'scope>,
        &mut Gradients<'scope>
    ) -> Result<(), TensorError> + 'scope>>,
}

pub(crate) struct Gradients<'scope> {
    gradients: HashMap<WeakTensor<'scope>, Tensor<'scope>>
}

impl<'scope> AutogradEncoder<'scope> {
    pub(crate) fn new() -> AutogradEncoder<'scope> {
        AutogradEncoder {
            tracked: HashSet::new(),
            operations: Vec::new()
        }
    }
}

impl<'scope> Gradients<'scope> {
    pub(crate) fn new() -> Self {
        Gradients {
            gradients: HashMap::new(),
        }
    }

    pub(crate) fn remove(
        &mut self,
        tensor: WeakTensor<'scope>
    ) -> Option<Tensor<'scope>> {
        self.gradients.remove(&tensor)
    } 

    pub(crate) fn insert(
        &mut self,
        encoder: &mut TensorEncoder<'scope>,
        tensor: WeakTensor<'scope>,
        gradient: Tensor<'scope>
    ) -> Result<(), TensorError> {
        let gradient = match self.gradients.get(&tensor) {
            Some(previous) => encoder.add(previous, &gradient)?,
            None => gradient
        };

        self.gradients.insert(tensor, gradient);
        Ok(())
    }
}

impl<'scope> AutogradEncoder<'scope> {
    pub(crate) fn require<const C: usize>(
        &mut self,
        deps: [&Tensor<'scope>; C],
        src: &Tensor<'scope>
    ) -> bool {
        let weak = src.downgrade();
        let tracked = self.tracked.contains(&weak);

        if src.trainable() && !tracked {
            let src = src.clone();
            let weak = weak.clone();
            self.operations.push(Box::new(move |encoder, gradients| {
                match gradients.remove(weak) {
                    Some(gradient) => src.optimize(encoder, &gradient),
                    None => Ok(())
                }
            }));
        }

        if src.trainable() || tracked {
            self.tracked.insert(weak);
            for tensor in deps {
                self.tracked.insert(tensor.downgrade());
            }

            true
        } else {
            false
        }
    }

    pub(crate) fn encode(
        self,
        encoder: &mut TensorEncoder<'scope>,
        loss: Tensor<'scope>
    ) -> Result<(), TensorError> {
        if loss.shape() != 1u32.shape() {
            return Err(TensorError::InvalidLossShape)
        }
        let mut gradients = Gradients::new();
        let one = encoder.ones(1)?;
        gradients.insert(encoder, loss.downgrade(), one)?;

        for operation in self.operations.into_iter().rev() {
            operation(encoder, &mut gradients)?;
        }

        Ok(())
    }

    pub(crate) fn backwards(
        &mut self,
        callback: impl FnOnce(
            &mut TensorEncoder<'scope>,
            &mut Gradients<'scope>
        ) -> Result<(), TensorError> + 'scope
    ) {
        self.operations.push(Box::new(callback))
    }
}