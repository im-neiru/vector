pub(in crate::renderer) trait Target {
    fn resize(
        &mut self,
        device: &wgpu::Device,
        width: u32,
        height: u32,
    );

    fn get_output(
        &self,
    ) -> Result<
        (Option<wgpu::SurfaceTexture>, wgpu::TextureView),
        wgpu::SurfaceError,
    >;

    fn format(&self) -> wgpu::TextureFormat;

    fn projection(&self) -> super::uniforms::Projection;
}
