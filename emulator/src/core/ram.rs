use std::ops::{Index, IndexMut, Range};
use std::fs::File;
use std::io;
use std::io::*;

const RAM_SIZE: usize = 0x4000;

pub struct RAM {
    mem: [u8; RAM_SIZE],
}

impl RAM {
    /*
     * Struct representing the RAM
     * Can be indexed with u16's but only the 14 LSB's are used
     * 2 MSB's are masked out, because all adresses >= 2^14 mirror the RAM
     *
     * fake ROM: 0000-1fff
     * RAM: 2000-3fff
     * RAM-Mirror: 4000-
     */
    pub fn new() -> Self {
        Self { mem: [0; RAM_SIZE] }
    }

    pub fn load_vec(&mut self, vec: Vec<u8>, start: u16) {
        let mut idx = start;
        for byte in vec {
            self[idx] = byte;
            idx += 1;
        }
    }

    pub fn load_file(&mut self, path: &str, start: u16) -> io::Result<()> {
        let mut f = File::open(path)?;
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes)?;
        Ok(())
    }
}

impl Index<u16> for RAM {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.mem[(index & 0x3fff) as usize]
    }
}

impl IndexMut<u16> for RAM {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.mem[(index & 0x3fff) as usize]
    }
}

impl Index<Range<usize>> for RAM {
    type Output = [u8];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.mem[range]
    }
}

#[cfg(test)]
mod ram_tests {
    use super::*;

    #[test]
    fn test_ram() {
        let mut r = RAM::new();

        r[0] = 1;
        r[0x5132] = 69;
        assert_eq!(r[0], 1);
        assert_eq!(r[0x4000], 1);
        assert_eq!(r[0x1132], 69);

        r[1] = 2; r[2] = 3; r[3] = 4; r[4] = 5;
        let slice = &r[0..5];
        assert_eq!(slice, &[1, 2, 3, 4, 5]);
    }
}
