#![no_std]
#![allow(unexpected_cfgs)]

#[cfg(any(target_arch = "spirv", spirv))]
pub mod add;

pub type Shape = [u32; 8];