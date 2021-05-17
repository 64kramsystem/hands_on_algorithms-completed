use crate::d5_2_hashmap_from_scratch::hash;

use std::{borrow::Borrow, hash::Hash};

#[derive(Debug)]
pub struct BucketList<K, V> {
    seed: u64,
    len: usize,
    buckets: Vec<Vec<(K, V)>>,
}

impl<K: Hash + Eq, V> BucketList<K, V> {
    pub fn new() -> Self {
        Self {
            seed: rand::random(),
            len: 0,
            buckets: vec![vec![]],
        }
    }

    // Returns the length of the bucket used.
    //
    pub fn push(&mut self, key: K, value: V) -> usize {
        let bucket = hash(self.seed, &key) as usize % self.buckets.len();
        self.buckets[bucket].push((key, value));
        self.len += 1;
        self.buckets[bucket].len()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket = hash(self.seed, &key) as usize % self.buckets.len();

        for (bucket_key, bucket_value) in &self.buckets[bucket] {
            if key == bucket_key.borrow() {
                return Some(bucket_value);
            }
        }

        None
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket = hash(self.seed, &key) as usize % self.buckets.len();

        for (bucket_key, bucket_value) in &mut self.buckets[bucket] {
            if key == (bucket_key as &K).borrow() {
                return Some(bucket_value);
            }
        }

        None
    }

    pub fn set_bucket(&mut self, bucket_i: usize) -> Option<Vec<(K, V)>> {
        let ref_existing = self.buckets.get_mut(bucket_i)?;
        let mut result = vec![];

        std::mem::swap(&mut result, ref_existing);

        self.len -= result.len();

        Some(result)
    }

    pub fn set_buckets(&mut self, num: usize) {
        for _ in self.buckets.len()..num {
            self.buckets.push(vec![]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_get() {
        let mut list = BucketList::new();

        list.push("foo", 123);
        list.push("bar", 456);

        assert_eq!(list.get("foo"), Some(&123));
        assert_eq!(list.get("bar"), Some(&456));
    }

    #[test]
    fn test_get_mut() {
        let mut list = BucketList::new();

        list.push("foo", 123);

        let value_mut = list.get_mut("foo").unwrap();
        *value_mut = 456;

        assert_eq!(list.get("foo"), Some(&456));
    }
}
