use shaders::{UnaryOperation, UnaryParameters};

use crate::{Tensor, TensorEncoder, TensorError};

impl<'scope> TensorEncoder<'scope> {
    pub fn negate(
        &mut self,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {
        self.unary(
            UnaryOperation::NEGATE,
            operand,
        )
    } 

    pub fn absolute(
        &mut self,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {
        self.unary(
            UnaryOperation::ABSOLUTE,
            operand,
        )
    }

    pub fn reciprocal(
        &mut self,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {
        self.unary(
            UnaryOperation::RECIPROCAL,
            operand,
        )
    }

    pub fn square_root(
        &mut self,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {
        self.unary(
            UnaryOperation::SQUARE_ROOT,
            operand,
        )
    }

    pub fn reciprocal_square_root(
        &mut self,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {
        self.unary(
            UnaryOperation::RECIPROCAL_SQUARE_ROOT,
            operand,
        )
    }

    pub fn exponential(
        &mut self,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {
        self.unary(
            UnaryOperation::EXPONENTIAL,
            operand,
        )
    }

    pub fn logarithm(
        &mut self,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {
        self.unary(
            UnaryOperation::LOGARITHM,
            operand,
        )
    }

    pub(crate) fn copy(
        &mut self,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {
        self.unary(
            UnaryOperation::COPY,
            operand,
        )
    }

    fn unary(
        &mut self,
        operation: UnaryOperation,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {

        let mut params = UnaryParameters {
            length: 1,
            operation
        };

        let mut zero = false;
        for dimension in operand.shape() {
            params.length = params.length.checked_mul(dimension)
                .ok_or(TensorError::OversizedTensor)?;
            zero |= dimension == 0;
        }

        let output = self.temp(operand.shape())?;
        if zero { return Ok(output) }   

        let compute_pass = self.encoder.compute(
            &mut self.pipelines.unary,
            super::shaders::unary::unary_main,
            &params
        );

        operand.bind(compute_pass, 1, true);
        output.bind(compute_pass, 2, false);
        let num_workgroups = params.length.div_ceil(256);

        if num_workgroups != 0 && num_workgroups <= u16::MAX as u32 {
            compute_pass.dispatch_workgroups(num_workgroups, 1, 1); 
        } else if num_workgroups != 0 {
            let floor = num_workgroups.isqrt();
            let x = floor + (floor * floor != num_workgroups) as u32;
            let y = num_workgroups.div_ceil(x);
            compute_pass.dispatch_workgroups(x, y, 1);
        }

        Ok(output)
    }
}
