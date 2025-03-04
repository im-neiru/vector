pub(crate) struct Headless {
    pub(super) width: u32,
    pub(super) height: u32,
}

impl super::Target for Headless {
    fn resize(
        &mut self,
        _: &wgpu::Device,
        width: u32,
        height: u32,
    ) {
        if (self.width == width && self.height == height)
            || width == 0
            || height == 0
        {
            return;
        }

        self.width = width;
        self.height = height;
    }

    fn get_output(
        &self,
    ) -> Result<
        (Option<wgpu::SurfaceTexture>, wgpu::TextureView),
        wgpu::SurfaceError,
    > {
        // TODO:
    }
}

impl Headless {
    pub(super) fn new(
        width: u32,
        height: u32,
    ) -> Box<dyn super::Target> {
        Box::new(Self { width, height })
    }
}
