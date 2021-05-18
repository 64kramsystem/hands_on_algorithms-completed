use crate::d5_3_bucket_list::BucketList;
use std::{borrow::Borrow, hash::Hash};

const BSIZE: usize = 8;
const BGROW: usize = 8;

// use std::{borrow::Borrow, hash::Hash};
//
#[derive(Debug)]
pub struct HMap<K, V> {
    num_moved: usize,
    main: BucketList<K, V>,
    grow: BucketList<K, V>,
}

impl<K: Hash + Eq, V> HMap<K, V> {
    pub fn new() -> Self {
        Self {
            num_moved: 0,
            main: BucketList::new(),
            grow: BucketList::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(existing_value) = self.main.get_mut(&key) {
            *existing_value = value;
        } else if let Some(existing_value) = self.grow.get_mut(&key) {
            *existing_value = value;
        } else if self.num_moved > 0 {
            self.grow.push(key, value);
            self.move_bucket();
        } else if self.main.push(key, value) > BSIZE / 2 {
            self.move_bucket();
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.main.get(key).or_else(|| self.grow.get(key))
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if let Some(value) = self.main.get_mut(key) {
            Some(value)
        } else {
            self.grow.get_mut(key)
        }
    }

    pub fn len(&self) -> usize {
        self.main.len + self.grow.len
    }

    pub fn move_bucket(&mut self) {
        if self.num_moved > 0 {
            self.grow
                .increase_buckets_num_to(self.main.buckets.len() + BGROW);
        }

        if let Some(old_bucket_entries) = self.main.remove_bucket_entries(self.num_moved) {
            for (key, value) in old_bucket_entries {
                self.grow.push(key, value);
            }
            self.num_moved += 1;
        } else {
            // If all buckets have been moved, then grow becomes the new main.
            //
            std::mem::swap(&mut self.main, &mut self.grow);
            self.num_moved = 0;
        }
    }
}
