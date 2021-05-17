use std::hash::{Hash, Hasher};

pub struct MHash {
    prev: u8,
    n: u128,
}

impl Hasher for MHash {
    fn finish(&self) -> u64 {
        self.n as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.n = ((self.n + 11) * (*byte as u128 + 13) + (byte ^ self.prev) as u128)
                % (std::u64::MAX as u128);
            self.prev = *byte;
        }
    }
}

pub fn hash<H: Hash>(seed: u64, value: H) -> u64 {
    let mut hasher = MHash { n: 0, prev: 0 };
    hasher.write_u64(seed);
    value.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashing_strings() {
        let value_hash = hash(55, "cat");

        assert_eq!(value_hash, hash(55, "cat"));
        assert_ne!(value_hash, hash(55, "tac"));
    }

    #[test]
    fn test_hashing_numbers() {
        let mut previous_hash = 0;

        for value in 0..10000 {
            let value_hash = hash(55, value);
            assert_ne!(value_hash, previous_hash);
            previous_hash = value_hash;
        }
    }
}
