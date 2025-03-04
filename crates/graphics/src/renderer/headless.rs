pub(crate) struct Headless {
    extent: wgpu::Extent3d,
    texture: wgpu::Texture,
}

impl super::Target for Headless {
    fn resize(
        &mut self,
        _: &wgpu::Device,
        width: u32,
        height: u32,
    ) {
        if (self.extent.width == width
            && self.extent.height == height)
            || width == 0
            || height == 0
        {
            return;
        }

        self.extent.width = width;
        self.extent.height = height;
    }

    fn get_output(
        &self,
    ) -> Result<
        (Option<wgpu::SurfaceTexture>, wgpu::TextureView),
        wgpu::SurfaceError,
    > {
        let view = self.texture.create_view(
            &wgpu::TextureViewDescriptor::default(),
        );

        Ok((None, view))
    }
}

impl Headless {
    pub(super) fn create(
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) -> Box<dyn super::Target> {
        let extent = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture =
            device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Headless Render Target"),
                size: extent,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            });

        Box::new(Self { texture, extent })
    }
}
