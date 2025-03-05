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

    #[inline]
    fn format(&self) -> wgpu::TextureFormat {
        self.config.format
    }
}

impl<'a> Surfaced<'a> {
    pub(super) fn create(
        surface: wgpu::Surface<'a>,
        adapter: &wgpu::Adapter,
        width: u32,
        height: u32,
    ) -> Box<dyn super::Target + 'a> {
        let surface_caps = surface.get_capabilities(adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Box::new(Self { surface, config })
    }
}
