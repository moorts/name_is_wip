use std::ops::{Index, IndexMut};

const RAM_SIZE: usize = 0x4000;

struct RAM {
    mem: [u8; RAM_SIZE],
}

impl RAM {
    fn new() -> Self {
        Self { mem: [0; RAM_SIZE] }
    }
}

impl Index<u16> for RAM {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.mem[(index & 0xfff) as usize]
    }
}

impl IndexMut<u16> for RAM {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.mem[(index & 0xfff) as usize]
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
        assert_eq!(r[0], 1);
        assert_eq!(r[0x4000], 1);
    }
}
