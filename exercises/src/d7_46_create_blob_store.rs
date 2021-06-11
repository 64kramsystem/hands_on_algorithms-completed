use std::{
    fs::{File, OpenOptions},
    io::{Seek, SeekFrom},
    path::Path,
};

use crate::{
    d7_44_blob_data_file::BlobError,
    d7_45_convert_any_data_to_bytes::{read_value, write_value},
};

/// Control size; size of the initial data.
///
const CONT_SIZE: u64 = 32;
const ELEMS_LOC: u64 = 24;

pub struct BlobStore {
    file: File,
    hseed: u64,
    block_size: u64,
    nblocks: u64,
    elems: u64,
}

impl BlobStore {
    pub fn new(fname: &str, block_size: u64, nblocks: u64) -> Result<Self, BlobError> {
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .read(true)
            .open(fname)?;

        file.set_len(CONT_SIZE + block_size * nblocks)?;

        let hseed = rand::random();
        let elems = 0;

        write_value(&mut file, hseed)?;
        write_value(&mut file, block_size)?;
        write_value(&mut file, nblocks)?;
        write_value(&mut file, elems)?;

        for x in 0..nblocks {
            file.seek(SeekFrom::Start(CONT_SIZE + x * block_size))?;

            write_value(&mut file, 0)?; // key len; 0 = no items

            // What is 16??? Author says data already written at the front, but we wrote only a u64.
            //
            write_value(&mut file, block_size - 16)?;
        }

        Ok(Self {
            file,
            hseed,
            block_size,
            nblocks,
            elems,
        })
    }

    pub fn open(fname: &str) -> Result<Self, BlobError> {
        let mut file = OpenOptions::new().write(true).read(true).open(fname)?;

        let hseed = read_value(&mut file)?;
        let block_size = read_value(&mut file)?;
        let nblocks = read_value(&mut file)?;
        let elems = read_value(&mut file)?;

        Ok(Self {
            file,
            hseed,
            block_size,
            nblocks,
            elems,
        })
    }

    pub fn new_or_open(fname: &str, block_size: u64, nblocks: u64) -> Result<Self, BlobError> {
        if Path::new(fname).exists() {
            Self::open(fname)
        } else {
            Self::new(fname, block_size, nblocks)
        }
    }

    pub fn inc_elems(&mut self, n: i32) -> Result<(), BlobError> {
        if n >= 0 {
            // The course doesn't consider the case of positive overflow.
            //
            self.elems = self.elems.checked_add(n as u64).unwrap();
        } else {
            self.elems = self.elems.checked_sub((-n) as u64).unwrap();
        }

        self.file.seek(SeekFrom::Start(ELEMS_LOC))?;
        write_value(&mut self.file, self.elems)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_file_operations() {
        let fname = "/tmp/test_file";

        let bs = BlobStore::new_or_open(fname, 1000, 10).unwrap();
        let bs2 = BlobStore::open(fname).unwrap();

        assert_eq!(bs2.block_size, bs.block_size);
    }
}
