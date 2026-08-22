#![no_std]
#![allow(unexpected_cfgs)]
#![cfg_attr(target_arch = "spirv", feature(asm_experimental_arch))]

use bytemuck::{Pod, Zeroable};

#[cfg(any(target_arch = "spirv", spirv))]
pub mod binary;

#[cfg(any(target_arch = "spirv", spirv))]
pub mod unary;

#[cfg(any(target_arch = "spirv", spirv))]
pub mod reduction;

pub type Shape = [u32; 8];

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct ReductionParameters {
    pub operation: ReductionOperation,
    pub cluster_shift: u32,

    pub inner_size: u32,
    pub reduction_size: u32,
    pub outer_size: u32
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct UnaryParameters {
    pub length: u32,
    pub operation: UnaryOperation
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct BinaryParameters {
    pub divisions: [FastDivU32; 7],
    pub num_dimensions: u32,
    pub length: u32,
    pub masks: u32,
    pub operation: BinaryOperation
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct FastDivU32 {
    pub divisor: u32,
    pub magic: u32,
    pub shift: u32,
    pub pad: u32,
}

#[repr(transparent)]
#[derive(Pod, Zeroable, Clone, Copy, PartialEq)]
pub struct BinaryOperation(u32);

impl BinaryOperation {
    pub const ADD: Self = Self(0);
    pub const SUBTRACT: Self = Self(1);
    pub const MULTIPLY: Self = Self(2);
    pub const DIVIDE: Self = Self(3);
    pub const POWER: Self = Self(4);
    pub const MINIMUM: Self = Self(5);
    pub const MAXIMUM: Self = Self(6);
    pub const REMAINDER: Self = Self(7);
}

#[repr(transparent)]
#[derive(Pod, Zeroable, Clone, Copy, PartialEq)]
pub struct UnaryOperation(u32);

impl UnaryOperation {
    pub const NEGATE: Self = Self(0);
    pub const ABSOLUTE: Self = Self(1);
    pub const RECIPROCAL: Self = Self(2);
    pub const SQUARE_ROOT: Self = Self(3);
    pub const RECIPROCAL_SQUARE_ROOT: Self = Self(4);
    pub const EXPONENTIAL: Self = Self(5);
    pub const LOGARITHM: Self = Self(6);
    pub const COPY: Self = Self(7);
}

#[repr(transparent)]
#[derive(Pod, Zeroable, Clone, Copy, PartialEq)]
pub struct ReductionOperation(u32);

impl ReductionOperation {
    pub const SUM: Self = Self(0);
}

