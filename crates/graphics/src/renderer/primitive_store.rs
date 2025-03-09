use std::any::TypeId;

use super::{
    binding_group_layouts::BindingGroupLayouts,
    primitives::{Primitive, PrimitiveState},
};

pub struct PrimitiveStore {
    buckets: Vec<Bucket>,
}

pub struct Bucket {
    type_id: TypeId,
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
                let new_bucket = Bucket {
                    type_id,
                    primitives: vec![Box::new(state)],
                };
                self.buckets.insert(index, new_bucket);
            }
        }

        Ok(())
    }
}
