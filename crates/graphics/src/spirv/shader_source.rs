use ash::vk::TaggedStructure;

pub struct ShaderSource<'a> {
    pub(crate) id: u32,
    pub(super) bytes: &'a [u8],
}

#[macro_export(local_inner_macros)]
macro_rules! include_spirv {
    ($id:literal, $path:expr) => {
        super::ShaderSource {
            id: $id,
            bytes: ::std::include_bytes!($path),
        }
    };
}

impl<'a> ShaderSource<'a> {
    #[inline]
    pub(crate) fn create_info(
        &'a self,
    ) -> ash::vk::ShaderModuleCreateInfo<'a> {
        ash::vk::ShaderModuleCreateInfo {
            s_type:
                ash::vk::ShaderModuleCreateInfo::STRUCTURE_TYPE,
            p_next: std::ptr::null(),
            flags: ash::vk::ShaderModuleCreateFlags::empty(),
            code_size: self.bytes.len(),
            p_code: self.bytes.as_ptr() as *const u32,
            _marker: std::marker::PhantomData,
        }
    }
}
