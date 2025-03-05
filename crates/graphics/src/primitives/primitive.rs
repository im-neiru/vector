pub trait Primitive {
    type State: PrimitiveState;

    fn create_state(
        self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        u_transform: &wgpu::Buffer,
        u_transform_type: wgpu::BindingType,
    ) -> Self::State;
}

pub trait PrimitiveState {
    fn draw(&mut self, render_pass: &mut wgpu::RenderPass<'_>);
}
