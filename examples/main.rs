use std::iter::repeat;

use wgpu::{Backends, DeviceDescriptor, ExperimentalFeatures, Instance, InstanceDescriptor, InstanceFlags, MemoryHints, PollType, PowerPreference, RequestAdapterOptions, Trace};
use pollster::FutureExt;
use wgpu_tensor::{OPTIONAL_FEATURES, REQUIRED_DOWNLEVEL_FLAGS, REQUIRED_LIMITS, PrintTensorReader, Tensor, TensorContext};

fn main() {
    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::PRIMARY,
        backend_options: Default::default(),
        display: None,
        flags: InstanceFlags::ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER,
        memory_budget_thresholds: Default::default()
    });

    let adapter = instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        apply_limit_buckets: false,
        compatible_surface: None,
        force_fallback_adapter: false
    }).block_on().expect("Failed to find an appropriate adapter");

    let downlevel_flags = adapter.get_downlevel_capabilities().flags;
    assert!(downlevel_flags.contains(REQUIRED_DOWNLEVEL_FLAGS));

    let (device, queue) = adapter.request_device(&DeviceDescriptor {
        label: None,
        required_features: adapter.features() & OPTIONAL_FEATURES,
        required_limits: REQUIRED_LIMITS.clone(),
        experimental_features: ExperimentalFeatures::disabled(),
        memory_hints: MemoryHints::MemoryUsage,
        trace: Trace::Off
    }).block_on().expect("Failed to create device");

    let mut context = TensorContext::new(device.clone());
    let tensor_a = Tensor::new(&mut context, 4).unwrap();
    let tensor_b = Tensor::new(&mut context, 4).unwrap();
 
    let mut encoder = device.create_command_encoder(&Default::default());
    context.infer(&mut encoder, |encoder| {
        encoder.write(&tensor_a, repeat(2.0).take(4));
        encoder.write(&tensor_b, repeat(10.0).take(4));

        let output = encoder.add(&tensor_a, &tensor_b)?;
        let output = encoder.reciprocal_square_root(&output)?;
        let output = encoder.sum(&output, 0)?;

        Ok(encoder.read(&output, PrintTensorReader::new()))
    }).unwrap();

    let index = queue.submit(Some(encoder.finish()));
    device.poll(PollType::Wait {
        submission_index: Some(index),
        timeout: None
    }).unwrap();
}