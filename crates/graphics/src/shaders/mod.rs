const RECTANGLE: &str = include_str!("rectangle.wgsl");

pub fn create_rectangle(
    device: &wgpu::Device,
) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("shader.rectangle"),
        source: wgpu::ShaderSource::Wgsl(RECTANGLE.into()),
    })
}
