#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use std::time::Instant;

use wgpu::util::DeviceExt;

pub async fn create_device_and_queue() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(), ..Default::default()
    });
    let adapter = instance.request_adapter(&Default::default()).await.unwrap();
    let features = adapter.features();
    adapter.request_device(&wgpu::DeviceDescriptor {
        label: None,
        features: features,
        limits: Default::default()
    }, None).await.unwrap()

}

/// run simple addition with values in GPU memory
#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run_simple_add(device: wgpu::Device, queue: wgpu::Queue) -> Vec<f32>{
    let start_instant = Instant::now();

    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    println!("shader compilation {:?}", start_instant.elapsed());

    let input_f = &[1.0f32, 2.0f32];
    let input: &[u8] = bytemuck::bytes_of(input_f);

    // prolly the storage memory in GPU
    let input_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: input,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    // staging memory
    let output_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: input.len() as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&compute_pipeline_layout),
        module: &cs_module,
        entry_point: "main",
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: input_buf.as_entire_binding(),
        }],
    });

    let mut encoder = device.create_command_encoder(&Default::default());

    // total threads running will be,
    // workgroups_x * workgroups_y * workgroups_z * workgroup_size_x * workgroup_size_y * workgroup_size_z 
    // So here it will be 2 * 1
    {
        let mut cpass = encoder.begin_compute_pass(&Default::default());
        cpass.set_pipeline(&pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.dispatch_workgroups(input_f.len() as u32, 1, 1);
    }

    encoder.copy_buffer_to_buffer(&input_buf, 0, &output_buf, 0, input.len() as u64);


    queue.submit(Some(encoder.finish()));

    let buf_slice = output_buf.slice(..);
    let _buf_future = buf_slice.map_async(wgpu::MapMode::Read, |_| ());
    device.poll(wgpu::Maintain::Wait);

    let data_raw = &buf_slice.get_mapped_range();
    let data: &[f32] = bytemuck::cast_slice(data_raw);

    data.to_vec()
}