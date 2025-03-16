use std::hash::Hash;

use ash::vk::TaggedStructure;

pub struct ShaderSource<'a, T>
where
    T: Hash + PartialEq + PartialOrd + Eq + Ord,
{
    pub(crate) id: T,
    pub(super) words: &'a [u32],
}

impl<'a, T> ShaderSource<'a, T>
where
    T: Hash + PartialEq + PartialOrd + Eq + Ord,
{
    #[inline]
    pub(crate) fn create_info(
        &'a self,
    ) -> ash::vk::ShaderModuleCreateInfo<'a> {
        ash::vk::ShaderModuleCreateInfo {
            s_type:
                ash::vk::ShaderModuleCreateInfo::STRUCTURE_TYPE,
            p_next: std::ptr::null(),
            flags: ash::vk::ShaderModuleCreateFlags::empty(),
            code_size: self.words.len() << 2,
            p_code: self.words.as_ptr(),
            _marker: std::marker::PhantomData,
        }
    }
}
