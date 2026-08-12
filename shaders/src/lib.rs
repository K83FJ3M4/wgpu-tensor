#![no_std]
#![allow(unexpected_cfgs)]

use bytemuck::{Pod, Zeroable};

#[cfg(any(target_arch = "spirv", spirv))]
pub mod add;

pub type Shape = [u32; 8];

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct BinaryParameters {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32
}