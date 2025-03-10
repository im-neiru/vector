use crate::renderer::{BindingGroupLayouts, Pipelines};

pub trait Primitive {
    type State: PrimitiveState;
    type Mutator;

    fn get_pipeline(
        pipelines: &Pipelines,
    ) -> std::sync::Arc<wgpu::RenderPipeline>;

    fn create_state(
        self,
        device: &wgpu::Device,
        projection_buffer: &wgpu::Buffer,
        binding_group_layouts: &mut BindingGroupLayouts,
    ) -> logging::Result<Self::State>;
}

pub trait PrimitiveState {
    fn draw(
        &mut self,
        render_pass: &mut wgpu::RenderPass<'_>,
        binding_group_layouts: &BindingGroupLayouts,
    );
}
