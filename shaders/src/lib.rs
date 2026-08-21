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
    pub mask_a: u32,
    pub mask_b: u32
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct FastDivU32 {
    pub divisor: u32,
    pub magic: u32,
    pub shift: u32,
    pub pad: u32,
}