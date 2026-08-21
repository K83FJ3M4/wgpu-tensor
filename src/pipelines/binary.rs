use bytemuck::Zeroable;
use shaders::{BinaryOperation, BinaryParameters, FastDivU32, Shape};
use crate::{Tensor, TensorEncoder, TensorError};

impl<'scope> TensorEncoder<'scope> {
    pub fn add(
        &mut self,
        operand_one: &Tensor,
        operand_two: &Tensor
    ) -> Result<Tensor<'scope>, TensorError> {
        self.binary(
            BinaryOperation::ADD,
            operand_one,
            operand_two,
        )
    }

    pub fn subtract(
        &mut self,
        operand_one: &Tensor,
        operand_two: &Tensor
    ) -> Result<Tensor<'scope>, TensorError> {
        self.binary(
            BinaryOperation::SUBTRACT,
            operand_one,
            operand_two,
        )
    }

    pub fn multiply(
        &mut self,
        operand_one: &Tensor,
        operand_two: &Tensor
    ) -> Result<Tensor<'scope>, TensorError> {
        self.binary(
            BinaryOperation::MULTIPLY,
            operand_one,
            operand_two,
        )
    }

    pub fn divide(
        &mut self,
        operand_one: &Tensor,
        operand_two: &Tensor
    ) -> Result<Tensor<'scope>, TensorError> {
        self.binary(
            BinaryOperation::DIVIDE,
            operand_one,
            operand_two,
        )
    }

    pub fn power(
        &mut self,
        operand_one: &Tensor,
        operand_two: &Tensor
    ) -> Result<Tensor<'scope>, TensorError> {
        self.binary(
            BinaryOperation::POWER,
            operand_one,
            operand_two,
        )
    }

    pub fn minimum(
        &mut self,
        operand_one: &Tensor,
        operand_two: &Tensor
    ) -> Result<Tensor<'scope>, TensorError> {
        self.binary(
            BinaryOperation::MINIMUM,
            operand_one,
            operand_two,
        )
    }

    pub fn maximum(
        &mut self,
        operand_one: &Tensor,
        operand_two: &Tensor
    ) -> Result<Tensor<'scope>, TensorError> {
        self.binary(
            BinaryOperation::MAXIMUM,
            operand_one,
            operand_two,
        )
    }

    pub fn remainder(
        &mut self,
        operand_one: &Tensor,
        operand_two: &Tensor
    ) -> Result<Tensor<'scope>, TensorError> {
        self.binary(
            BinaryOperation::REMAINDER,
            operand_one,
            operand_two,
        )
    }

    fn binary(
        &mut self,
        operation: BinaryOperation,
        operand_one: &Tensor,
        operand_two: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {

        let mut shape = operand_one.shape();

        let mut zero = false;
        for (dst, src) in shape.iter_mut().zip(operand_two.shape()) {
            *dst = Self::boradcast_dimension(*dst, src)?;
            zero |= *dst == 0;
        }

        let output = self.temp(shape)?;
        if zero { return Ok(output) }
        
        let params = Self::create_binary_params(
            operand_one.shape(),
            operand_two.shape(),
            operation
        )?;

        let compute_pass = self.encoder.compute(
            &mut self.pipelines.binary,
            super::shaders::binary::binary_main,
            &params
        );

        operand_one.bind(compute_pass, 1, true);
        operand_two.bind(compute_pass, 2, true);
        output.bind(compute_pass, 3, false);
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

    fn create_binary_params(
        shape_a: Shape,
        shape_b: Shape,
        operation: BinaryOperation
    ) -> Result<BinaryParameters, TensorError> {
        let mut accumulator = Option::<[u32; 2]>::None;
        let mut params = BinaryParameters::zeroed();
        params.operation = operation;
        params.length = 1;

        for (a, b) in shape_a.into_iter().zip(shape_b) {
            Self::boradcast_dimension(a, b)?;
            if a == 1 && b == 1 { continue }

            if let Some([acc_a, acc_b]) = accumulator.as_mut()
                .filter(|accumulator| {
                    accumulator.map(|dimension| dimension != 1)
                    .eq(&[a, b].map(|dimension| dimension != 1))
                }
            ) {
                let error = TensorError::OversizedDispatch;
                *acc_a = acc_a.checked_mul(a).ok_or(error)?;
                *acc_b = acc_b.checked_mul(b).ok_or(error)?;
            } else {
                if let Some([acc_a, acc_b]) = accumulator.replace([a, b]) {
                    Self::push_dimension(&mut params, acc_a, acc_b)?;
                }
            }
        }

        if let Some([a, b]) = accumulator {
            Self::push_dimension(&mut params, a, b)?;
        } 

        Ok(params)
    } 

    fn create_div(divisor: u32) -> FastDivU32 {
        assert!(divisor != 0);

        if divisor == 1 {
            return FastDivU32 {
                divisor,
                magic: 0,
                shift: 0,
                pad: 0 
            }
        }

        let floor_log2 = 31 - divisor.leading_zeros();
        if divisor.is_power_of_two() {
            return FastDivU32 {
                divisor: divisor,
                magic: 0,
                shift: floor_log2 - 1,
                pad: 0
            }
        }

        let numerator = 1u64 << (32 + floor_log2);
        let mut proposed_m = (numerator / divisor as u64) as u32;
        let remainder = (numerator % divisor as u64) as u32;
        proposed_m = proposed_m.wrapping_add(proposed_m);
        let twice_remainder = remainder.wrapping_add(remainder);

        if twice_remainder >= divisor || twice_remainder < remainder {
            proposed_m = proposed_m.wrapping_add(1);
        }

        FastDivU32 {
            divisor,
            magic: proposed_m.wrapping_add(1),
            shift: floor_log2,
            pad: 0
        }
    }

    fn push_dimension(
        params: &mut BinaryParameters,
        a: u32,
        b: u32
    ) -> Result<(), TensorError> {
        assert!((a == b) || (a == 1) || (b == 1));
        if (a == 1) && (b == 1) { return Ok(()) }

        let dimension = Self::boradcast_dimension(a, b)?;
        let index = params.num_dimensions as usize;
        if let Some(division) = params.divisions.get_mut(index) {
            if dimension != 0 {
                *division = Self::create_div(dimension);
            }
        }

        params.length = params.length.checked_mul(dimension)
            .ok_or(TensorError::OversizedDispatch)?;

        let mut masks = unpack_2x_u16(params.masks);
        masks[0] |= ((a != 1) as u32) << params.num_dimensions;
        masks[1] |= ((b != 1) as u32) << params.num_dimensions;
        params.masks = pack_2x_u16(masks);
        params.num_dimensions += 1;

        Ok(())
    }
    
    fn boradcast_dimension(a: u32, b: u32) -> Result<u32, TensorError> {
        if a == b {
            Ok(a)
        } else if a == 1 {
            Ok(b)
        } else if b == 1 {
            Ok(a)
        } else {
            Err(TensorError::ShapeMismatch)
        }
    }
}

#[inline]
pub fn pack_2x_u16([x, y]: [u32; 2]) -> u32 {
    (x & 0xFFFF) | ((y & 0xFFFF) << 16)
}

#[inline]
pub fn unpack_2x_u16(v: u32) -> [u32; 2] {
    [v & 0xFFFF, v >> 16]
}