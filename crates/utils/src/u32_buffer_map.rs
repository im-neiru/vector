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
    pub const fn new() -> Self {
        assert!(
            N.is_power_of_two(),
            "N must be a power of two for bitwise AND hashing to work correctly"
        );
        Self {
            buckets: [const { Vec::new() }; N],
        }
    }

    #[inline]
    const fn bucket_index(key: u32) -> usize {
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
        for entry in self.buckets[index].iter() {
            if entry.key == key {
                return Some(&entry.value);
            }
        }

        None
    }

    #[inline]
    pub fn get_mut(&mut self, key: u32) -> Option<&mut T> {
        let index = Self::bucket_index(key);

        for entry in self.buckets[index].iter_mut() {
            if entry.key == key {
                return Some(&mut entry.value);
            }
        }

        None
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
        let index = Self::bucket_index(key);

        for entry in self.buckets[index].iter() {
            if entry.key == key {
                return true;
            }
        }

        false
    }
}

impl<T, const N: usize> core::fmt::Debug for U32BufferMap<T, N>
where
    T: core::fmt::Debug,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let mut map = f.debug_map();
        for bucket in self.buckets.iter() {
            for entry in bucket {
                map.entry(&entry.key, &entry.value);
            }
        }

        map.finish()
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

#[test]
fn test_speed_u32_buffer_map() {
    use rand::Rng;

    let mut rng = rand::rng();

    for n in 1..=25 {
        println!("#{}: ", n);

        let sample_data: Vec<(u32, String)> = (0..(n * 5))
            .map(|_| {
                let key = rng.random::<u32>();
                (key, format!("gibberish_{}", key))
            })
            .collect();

        let start = std::time::Instant::now();
        {
            let mut u32_buffer_map =
                U32BufferMap::<String, 8>::new();

            for &(key, ref value) in &sample_data {
                u32_buffer_map.insert(key, value.clone());
            }

            for &(key, _) in &sample_data {
                let _ = u32_buffer_map.get(key);
            }
        }
        let duration_custom = start.elapsed();
        println!("U32BufferMap took: {:?}", duration_custom);

        let start = std::time::Instant::now();
        {
            let mut std_map = std::collections::HashMap::new();
            for &(key, ref value) in &sample_data {
                std_map.insert(key, value.clone());
            }
            for &(key, _) in &sample_data {
                let _ = std_map.get(&key);
            }
        }
        let duration_std = start.elapsed();
        println!("HashMap took: {:?}", duration_std);
    }
}
