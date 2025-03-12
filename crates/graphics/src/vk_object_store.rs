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
    pub(crate) unsafe fn use_object<
        F: FnOnce() -> logging::Result<T>,
    >(
        &mut self,
        key: K,
        constructor: F,
    ) -> logging::Result<&T> {
        let storage = self.0.get_or_insert(Vec::new());

        let index = match storage
            .binary_search_by(|entry| entry.key.cmp(&key))
        {
            Ok(index) => index,
            Err(dest) => {
                let object = constructor()?;

                storage.insert(dest, Entry { key, object });

                dest
            }
        };

        Ok(unsafe {
            &storage
                .get(index)
                .as_ref()
                .unwrap_unchecked()
                .object
        })
    }

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
