use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{d5_2_hashmap_from_scratch, d7_44_blob_data_file::BlobError};
use std::io;

pub fn read_value<V: DeserializeOwned, R: io::Read>(r: &mut R) -> Result<V, BlobError> {
    let result = bincode::deserialize_from(r)?;
    Ok(result)
}

pub fn write_value<V: Serialize, W: io::Write>(w: &mut W, v: V) -> Result<(), BlobError> {
    bincode::serialize_into(w, &v)?;
    Ok(())
}

pub struct Blob {
    k: Vec<u8>,
    v: Vec<u8>,
}

impl Blob {
    pub fn from<K: Serialize, V: Serialize>(k: &K, v: &V) -> Result<Blob, BlobError> {
        let k = bincode::serialize(k)?;
        let v = bincode::serialize(v)?;

        Ok(Blob { k, v })
    }

    pub fn write<W: io::Write>(&self, w: &mut W) -> Result<(), BlobError> {
        write_value(w, self.k.len())?;
        write_value(w, self.v.len())?;
        w.write_all(&self.k)?;
        w.write_all(&self.v)?;

        Ok(())
    }

    pub fn read<R: io::Read>(r: &mut R) -> Result<Blob, failure::Error> {
        let k_len: usize = read_value(r)?;
        let v_len: usize = read_value(r)?;

        let mut k = vec![0_u8; k_len];
        let mut v = vec![0_u8; v_len];

        r.read_exact(&mut k)?;
        r.read_exact(&mut v)?;

        Ok(Blob { k, v })
    }

    pub fn get_v<V: DeserializeOwned>(&self) -> Result<V, BlobError> {
        let result = bincode::deserialize(&self.v)?;
        Ok(result)
    }

    // Borrowed version of get_v().
    //
    pub fn get_v_brw<'d, V: Deserialize<'d>>(&'d self) -> Result<V, BlobError> {
        let result = bincode::deserialize(&self.v)?;
        Ok(result)
    }

    pub fn deserialize<V: DeserializeOwned>(bindata: &[u8]) -> Result<V, bincode::Error> {
        bincode::deserialize(bindata)
    }

    pub fn deserialize_b<'d, V: Deserialize<'d>>(bindata: &'d [u8]) -> Result<V, bincode::Error> {
        bincode::deserialize(bindata)
    }

    pub fn len(&self) -> usize {
        8 + 8 + self.k.len() + self.v.len()
    }

    pub fn k_hash(&self, seed: u64) -> u64 {
        d5_2_hashmap_from_scratch::hash(seed, &self.k)
    }

    pub fn key_match(&self, rhs: &Self) -> bool {
        self.k == rhs.k
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_derive::*;
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct Point<T> {
        x: T,
        y: T,
    }

    #[test]
    fn test_read_write_string() {
        let k: i32 = 87;
        let v = "hello world";
        let blob = Blob::from(&k, &v).unwrap();
        {
            let mut fout = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open("/tmp/t_rblob.dat")
                .unwrap();
            blob.write(&mut fout).unwrap();
        }

        let mut fin = std::fs::File::open("/tmp/t_rblob.dat").unwrap();
        let b2 = Blob::read(&mut fin).unwrap();
        let v2: String = b2.get_v().unwrap();
        assert_eq!(&v2, "hello world");

        //It's just Bytes

        let p: Point<i32> = b2.get_v().unwrap();
        assert_eq!(p, Point { x: 11, y: 0 });
    }

    #[test]
    pub fn test_ser64() {
        let ndat = bincode::serialize(&0u64).unwrap();
        assert_eq!(ndat.len(), 8);
    }
}
