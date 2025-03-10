use super::uniforms::Projection;

pub(crate) struct Headless {
    extent: wgpu::Extent3d,
    texture: wgpu::Texture,
}

impl super::Target for Headless {
    fn resize(
        &mut self,
        device: &wgpu::Device,
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

        self.texture =
            Self::create_texture(device, self.extent);
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

    #[inline]
    fn format(&self) -> wgpu::TextureFormat {
        self.texture.format()
    }

    #[inline]
    fn projection(&self) -> Projection {
        Projection::new(self.extent.width, self.extent.height)
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

        let texture = Self::create_texture(device, extent);

        Box::new(Self { texture, extent })
    }

    #[inline]
    fn create_texture(
        device: &wgpu::Device,
        extent: wgpu::Extent3d,
    ) -> wgpu::Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Headless Render Target"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        })
    }
}
