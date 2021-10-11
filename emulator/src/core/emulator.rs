use std::ops::{Index, IndexMut};

const RAM_SIZE: usize = 0x4000;

struct RAM {
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
    fn new() -> Self {
        Self { mem: [0; RAM_SIZE] }
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

struct Emulator {
    ram: RAM,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram() {
        let mut r = RAM::new();

        r[0] = 1;
        r[0x5132] = 69;
        assert_eq!(r[0], 1);
        assert_eq!(r[0x4000], 1);
        assert_eq!(r[0x1132], 69);
    }
}
