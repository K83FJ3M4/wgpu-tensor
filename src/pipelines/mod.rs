use std::collections::HashMap;

use bytemuck::{Pod, Zeroable};
use wgpu::{BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType, BufferSize, ComputePipeline, Device, ShaderStages};

use crate::{Shape, TensorError};

mod reduction;
mod binary;
mod matmul;
mod unary;
mod constant;

pub(crate) struct Pipelines {
    pub(crate) device: Device,
    pub(crate) tensor_input_layout: BindGroupLayout,
    pub(crate) tensor_output_layout: BindGroupLayout,
    pub(crate) param_layouts: ParamLayouts,

    reduction: Option<ComputePipeline>,
    constant: Option<ComputePipeline>,
    binary: Option<ComputePipeline>,
    matmul: Option<ComputePipeline>,
    unary: Option<ComputePipeline>,
}

pub(crate) struct ParamLayouts {
    cache: HashMap<usize, BindGroupLayout>
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct BroadcastInfo {
    divisions: [FastDivU32; 7]
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct FastDivU32 {
    divisor: u32,
    magic: u32,
    shift: u32,
    data: u32
}

impl Pipelines {
    pub(crate) fn new(device: Device) -> Pipelines {

        let tensor_input_layout_ty = BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: BufferSize::new(size_of::<u32>() as u64)
        };

        let tensor_input_layout = device.create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                label: Some("Tensor Input"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    visibility: ShaderStages::COMPUTE,
                    ty: tensor_input_layout_ty
                }]
            }
        );

        let tensor_output_layout_ty = BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only: false },
            has_dynamic_offset: false,
            min_binding_size: BufferSize::new(size_of::<u32>() as u64)
        };

        let tensor_output_layout = device.create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                label: Some("Tensor Output"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    visibility: ShaderStages::COMPUTE,
                    ty: tensor_output_layout_ty
                }]
            }
        );

        Pipelines {
            param_layouts: ParamLayouts::new(),
            tensor_input_layout,
            tensor_output_layout,
            device,

            reduction: None,
            constant: None,
            binary: None,
            matmul: None,
            unary: None
        }
    } 
}

impl ParamLayouts {
    fn new() -> ParamLayouts {
        ParamLayouts {
            cache: HashMap::new()
        }
    }

    pub(crate) fn get<T: Pod>(&mut self, device: &Device) -> &BindGroupLayout {
        let key = size_of::<T>();
        self.cache.entry(key).or_insert_with(|| {
            let param_ty = BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: true,
                min_binding_size: BufferSize::new(key as u64)
            };

            device.create_bind_group_layout(
                &BindGroupLayoutDescriptor {
                    label: Some(&format!("Parameters ({key})")),
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        count: None,
                        visibility: ShaderStages::COMPUTE,
                        ty: param_ty
                    }]
                }
            )
        })
    }
}

impl BroadcastInfo { 
    fn new(lhs: Shape, rhs: Shape) -> Result<BroadcastInfo, TensorError> {
        Self::with_prefix(lhs, rhs, &[])
    }

    fn with_prefix(
        lhs: Shape,
        rhs: Shape,
        prefix: &[u32]
    ) -> Result<BroadcastInfo, TensorError> {
        let mut accumulator = Option::<[u32; 2]>::None;
        let mut params = BroadcastInfo::zeroed();

        for dimension in prefix {
            params.push_output_dimension(*dimension);
        }

        for (a, b) in lhs.into_iter().zip(rhs).skip(prefix.len()) {
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

    fn push_output_dimension(&mut self, dimension: u32) {
        let index = *self.num_dimensions() as usize;
        if let Some(division) = self.divisions.get_mut(index) {
            if dimension != 0 {
                let target = Self::create_div(dimension);
                division.divisor = target.divisor;
                division.magic = target.magic;
                division.shift = target.shift;
            }
        }

        *self.num_dimensions() += 1;
    }

    fn create_div(divisor: u32) -> FastDivU32 {
        assert!(divisor != 0);

        if divisor == 1 {
            return FastDivU32 {
                divisor,
                magic: 0,
                shift: 0,
                data: 0
            }
        }

        let floor_log2 = 31 - divisor.leading_zeros();
        if divisor.is_power_of_two() {
            return FastDivU32 {
                divisor: divisor,
                magic: 0,
                shift: floor_log2 - 1,
                data: 0
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
            data: 0
        }
    }

    fn push_dimension(&mut self, lhs: u32, rhs: u32) -> Result<(), TensorError> {
        assert!((lhs == rhs) || (lhs == 1) || (rhs == 1));
        if (lhs == 1) && (rhs == 1) { return Ok(()) }

        let dimension = Self::boradcast_dimension(lhs, rhs)?;
        let index = *self.num_dimensions() as usize;
        if let Some(division) = self.divisions.get_mut(index) {
            if dimension != 0 {
                let target = Self::create_div(dimension);
                division.divisor = target.divisor;
                division.magic = target.magic;
                division.shift = target.shift;
            }
        }

        let mut masks = unpack_2x_u16(*self.masks());
        masks[0] |= ((lhs != 1) as u32) << *self.num_dimensions();
        masks[1] |= ((rhs != 1) as u32) << *self.num_dimensions();
        *self.masks() = pack_2x_u16(masks);
        *self.num_dimensions() += 1;

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

    fn num_dimensions(&mut self) -> &mut u32 {
        &mut self.divisions[0].data
    }

    fn masks(&mut self) -> &mut u32 {
        &mut self.divisions[1].data
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