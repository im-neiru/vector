use std::any::TypeId;

use super::{
    binding_group_layouts::BindingGroupLayouts,
    pipelines::Pipelines,
    primitives::{Primitive, PrimitiveState},
};

pub struct PrimitiveStore {
    buckets: Vec<RenderBucket>,
}

pub struct RenderBucket {
    type_id: TypeId,
    pipeline: std::sync::Arc<wgpu::RenderPipeline>,
    primitives: Vec<Box<dyn PrimitiveState>>,
}

impl PrimitiveStore {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
        }
    }

    pub fn add<S, P>(
        &mut self,
        device: &wgpu::Device,
        projection_buffer: &wgpu::Buffer,
        binding_group_layouts: &mut BindingGroupLayouts,
        pipelines: &Pipelines,
        primitive: P,
    ) -> logging::Result<()>
    where
        S: PrimitiveState + 'static,
        P: Primitive<State = S> + 'static,
    {
        let state = primitive.create_state(
            device,
            projection_buffer,
            binding_group_layouts,
        )?;

        let type_id = TypeId::of::<P>();

        match self.buckets.binary_search_by(|bucket| {
            bucket.type_id.cmp(&type_id)
        }) {
            Ok(index) => {
                self.buckets[index]
                    .primitives
                    .push(Box::new(state));
            }
            Err(index) => {
                let pipeline = P::get_pipeline(pipelines);

                let new_bucket = RenderBucket {
                    type_id,
                    pipeline,
                    primitives: vec![Box::new(state)],
                };
                self.buckets.insert(index, new_bucket);
            }
        }

        Ok(())
    }

    pub fn render(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        binding_group_layouts: &BindingGroupLayouts,
    ) {
        for bucket in self.buckets.iter_mut() {
            render_pass.set_pipeline(&bucket.pipeline);

            for primitive in bucket.primitives.iter_mut() {
                primitive
                    .draw(render_pass, binding_group_layouts);
            }
        }
    }
}
