pub struct VkObjectStore<K, T>(Option<Vec<Entry<K, T>>>)
where
    K: PartialEq + PartialOrd + Ord + Eq;

struct Entry<K, T>
where
    K: PartialEq + PartialOrd + Ord + Eq,
{
    key: K,
    object: T,
}

impl<K, T> Default for VkObjectStore<K, T>
where
    K: PartialEq + PartialOrd + Ord + Eq,
{
    fn default() -> Self {
        Self(None)
    }
}

impl<K, T> VkObjectStore<K, T>
where
    K: PartialEq + PartialOrd + Ord + Eq,
{
    #[inline]
    pub(crate) unsafe fn destroy<F: Fn(T)>(
        &mut self,
        destructor: F,
    ) {
        if let Some(entries) = self.0.take() {
            for entry in entries.into_iter() {
                destructor(entry.object);
            }
        }
    }
}
