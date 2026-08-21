#![no_std]
#![allow(unexpected_cfgs)]

use bytemuck::{Pod, Zeroable};

#[cfg(any(target_arch = "spirv", spirv))]
pub mod binary;

pub type Shape = [u32; 8];

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
