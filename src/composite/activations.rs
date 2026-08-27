use crate::{Tensor, TensorEncoder, TensorError};

impl<'scope> TensorEncoder<'scope> {
    pub fn relu(
        &mut self,
        input: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {
        let zero = self.zeros(1)?;
        self.maximum(input, &zero)
    }

    pub fn leaky_relu(
        &mut self,
        input: &Tensor<'scope>,
        negative_slope: f32,
    ) -> Result<Tensor<'scope>, TensorError> {
        let zero = self.zeros(1)?;
        let positive = self.maximum(input, &zero)?;
        let negative = self.minimum(input, &zero)?;
        let negative_slope = self.constant(negative_slope, 1)?;
        let negative = self.multiply(&negative, &negative_slope)?;
        self.add(&positive, &negative)
    }

    pub fn sigmoid(
        &mut self,
        input: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {
        let negative = self.negate(input)?;
        let exponential = self.exponential(&negative)?;
        let one = self.ones(1)?;
        let denominator = self.add(&one, &exponential)?;
        self.reciprocal(&denominator)
    }

    pub fn softmax(
        &mut self,
        input: &Tensor<'scope>,
        dimension: usize,
    ) -> Result<Tensor<'scope>, TensorError> {
        let maximum = self.max(input, dimension)?;
        let shifted = self.subtract(input, &maximum)?;
        let exponential = self.exponential(&shifted)?;
        let denominator = self.sum(&exponential, dimension)?;
        self.divide(&exponential, &denominator)
    }

    pub fn log_softmax(
        &mut self,
        input: &Tensor<'scope>,
        dimension: usize,
    ) -> Result<Tensor<'scope>, TensorError> {
        let maximum = self.max(input, dimension)?;
        let shifted = self.subtract(input, &maximum)?;
        let exponential = self.exponential(&shifted)?;
        let denominator = self.sum(&exponential, dimension)?;
        let log_denominator = self.logarithm(&denominator)?;
        self.subtract(&shifted, &log_denominator)
    }
}
