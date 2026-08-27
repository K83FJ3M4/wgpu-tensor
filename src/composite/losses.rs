use crate::{Tensor, TensorEncoder, TensorError};
use crate::tensor::AllDimensions;

impl<'scope> TensorEncoder<'scope> {
    pub fn mean_squared_error(
        &mut self,
        prediction: &Tensor<'scope>,
        target: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {
        validate_shapes(prediction, target)?;

        let difference = self.subtract(prediction, target)?;
        let squared = self.multiply(&difference, &difference)?;
        self.mean(&squared, AllDimensions)
    }

    pub fn mean_absolute_error(
        &mut self,
        prediction: &Tensor<'scope>,
        target: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {
        validate_shapes(prediction, target)?;

        let difference = self.subtract(prediction, target)?;
        let absolute = self.absolute(&difference)?;
        self.mean(&absolute, AllDimensions)
    }

    pub fn binary_cross_entropy_with_logits(
        &mut self,
        logits: &Tensor<'scope>,
        target: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {
        validate_shapes(logits, target)?;

        let zero = self.zeros(1)?;
        let positive = self.maximum(logits, &zero)?;
        let weighted_logits = self.multiply(logits, target)?;
        let linear = self.subtract(&positive, &weighted_logits)?;

        let magnitude = self.absolute(logits)?;
        let negative_magnitude = self.negate(&magnitude)?;
        let exponential = self.exponential(&negative_magnitude)?;
        let one = self.ones(1)?;
        let one_plus_exponential = self.add(&one, &exponential)?;
        let softplus = self.logarithm(&one_plus_exponential)?;

        let loss = self.add(&linear, &softplus)?;
        self.mean(&loss, AllDimensions)
    }

    pub fn categorical_cross_entropy_with_logits(
        &mut self,
        logits: &Tensor<'scope>,
        target: &Tensor<'scope>,
        class_dimension: usize,
    ) -> Result<Tensor<'scope>, TensorError> {
        validate_shapes(logits, target)?;

        let log_probabilities = self.log_softmax(logits, class_dimension)?;
        let weighted = self.multiply(target, &log_probabilities)?;
        let per_item = self.sum(&weighted, class_dimension)?;
        let per_item = self.negate(&per_item)?;
        self.mean(&per_item, AllDimensions)
    }
}

fn validate_shapes(
    prediction: &Tensor,
    target: &Tensor,
) -> Result<(), TensorError> {
    if prediction.shape() == target.shape() {
        Ok(())
    } else {
        Err(TensorError::ShapeMismatch)
    }
}