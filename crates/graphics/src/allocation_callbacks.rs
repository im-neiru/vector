use ash::vk::AllocationCallbacks;

const ALLOCATION_CALLBACKS_VALUE: Option<AllocationCallbacks> =
    None;

pub(crate) const ALLOCATION_CALLBACKS: Option<
    &AllocationCallbacks,
> = ALLOCATION_CALLBACKS_VALUE.as_ref();

// TODO: Track allocations
