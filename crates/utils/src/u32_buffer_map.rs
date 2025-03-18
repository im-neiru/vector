pub struct U32BufferMap<T, const N: usize> {
    buckets: [Vec<Entry<T>>; N],
}

struct Entry<T> {
    key: u32,
    value: T,
}

impl<T, const N: usize> U32BufferMap<T, N> {
    const DIVIDER: usize = N - 1;

    #[inline]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        assert!(
            N.is_power_of_two(),
            "N must be a power of two for bitwise AND hashing to work correctly"
        );
        Self {
            buckets: std::array::from_fn(|_| Vec::new()),
        }
    }

    #[inline]
    fn bucket_index(key: u32) -> usize {
        (key as usize) & Self::DIVIDER
    }

    #[inline]
    pub fn insert(&mut self, key: u32, value: T) -> Option<T> {
        let index = Self::bucket_index(key);
        let bucket = &mut self.buckets[index];
        for entry in bucket.iter_mut() {
            if entry.key == key {
                return Some(std::mem::replace(
                    &mut entry.value,
                    value,
                ));
            }
        }

        bucket.push(Entry { key, value });
        None
    }

    #[inline]
    pub fn get(&self, key: u32) -> Option<&T> {
        let index = Self::bucket_index(key);
        self.buckets[index]
            .iter()
            .find(|entry| entry.key == key)
            .map(|entry| &entry.value)
    }

    #[inline]
    pub fn get_mut(&mut self, key: u32) -> Option<&mut T> {
        let index = Self::bucket_index(key);
        self.buckets[index]
            .iter_mut()
            .find(|entry| entry.key == key)
            .map(|entry| &mut entry.value)
    }

    #[inline]
    pub fn remove(&mut self, key: u32) -> Option<T> {
        let index = Self::bucket_index(key);
        let bucket = &mut self.buckets[index];
        bucket
            .iter()
            .position(|entry| entry.key == key)
            .map(|pos| bucket.swap_remove(pos).value)
    }

    #[inline]
    pub fn contains_key(&self, key: u32) -> bool {
        self.get(key).is_some()
    }
}

#[test]
fn test_insert_get_remove() {
    let mut map = U32BufferMap::<String, 8>::new();

    assert_eq!(map.insert(42, "Lorem".to_string()), None);
    assert_eq!(map.insert(7, "Ipsum".to_string()), None);

    assert_eq!(map.get(42).map(String::as_str), Some("Lorem"));
    assert_eq!(map.get(7).map(String::as_str), Some("Ipsum"));
    assert!(map.get(100).is_none());

    assert_eq!(
        map.insert(42, "String1".to_string()),
        Some("Lorem".to_string())
    );
    assert_eq!(
        map.get(42).map(String::as_str),
        Some("String1")
    );

    assert_eq!(map.remove(42), Some(String::from("String1")));
    assert!(map.get(42).is_none());
}
