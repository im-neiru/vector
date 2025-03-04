pub(crate) struct Surfaced<'a> {
    pub(super) surface: wgpu::Surface<'a>,
    pub(super) config: wgpu::SurfaceConfiguration,
}

impl super::Target for Surfaced<'_> {
    fn resize(
        &mut self,
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) {
        if (self.config.width == width
            && self.config.height == height)
            || width == 0
            || height == 0
        {
            return;
        }

        self.config.width = width;
        self.config.height = height;

        self.surface.configure(device, &self.config);
    }

    fn get_output(
        &self,
    ) -> Result<
        (Option<wgpu::SurfaceTexture>, wgpu::TextureView),
        wgpu::SurfaceError,
    > {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(
            &wgpu::TextureViewDescriptor::default(),
        );

        Ok((Some(output), view))
    }
}

impl<'a> Surfaced<'a> {
    pub(super) fn new(
        surface: wgpu::Surface<'a>,
        config: wgpu::SurfaceConfiguration,
    ) -> Box<dyn super::Target + 'a> {
        Box::new(Self { surface, config })
    }
}
