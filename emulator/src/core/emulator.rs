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

#[repr(C)]
union Register {
    bytes: (u8, u8),
    value: u16,
}

impl Register {
    fn new() -> Self {
        Register { value: 0 }
    }
}

struct RegisterArray {
    wz: Register,
    bc: Register,
    de: Register,
    hl: Register,
}

impl RegisterArray {
    fn new() -> Self {
        RegisterArray { wz: Register::new(), bc: Register::new(), de: Register::new(), hl: Register::new() }
    }
}

impl Index<char> for RegisterArray {
    type Output = u8;

    fn index(&self, index: char) -> &Self::Output {
        unsafe {
            match index {
                'w' => &self.wz.bytes.1,
                'z' => &self.wz.bytes.0,
                'b' => &self.bc.bytes.1,
                'c' => &self.bc.bytes.0,
                'd' => &self.de.bytes.1,
                'e' => &self.de.bytes.0,
                'h' => &self.de.bytes.1,
                'l' => &self.de.bytes.0,
                _ => panic!("Invalid register")
            }
        }
    }
}

impl IndexMut<char> for RegisterArray {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        unsafe {
            match index {
                'w' => &mut self.wz.bytes.1,
                'z' => &mut self.wz.bytes.0,
                'b' => &mut self.bc.bytes.1,
                'c' => &mut self.bc.bytes.0,
                'd' => &mut self.de.bytes.1,
                'e' => &mut self.de.bytes.0,
                'h' => &mut self.de.bytes.1,
                'l' => &mut self.de.bytes.0,
                _ => panic!("Invalid register")
            }
        }
    }
}

impl Index<&str> for RegisterArray {
    type Output = u16;

    fn index(&self, index: &str) -> &Self::Output {
        unsafe {
            match index {
                "wz" => &self.wz.value,
                "bc" => &self.bc.value,
                "de" => &self.de.value,
                "hl" => &self.hl.value,
                _ => panic!("Invalid register pair")
            }
        }
    }
}

impl IndexMut<&str> for RegisterArray {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        unsafe {
            match index {
                "wz" => &mut self.wz.value,
                "bc" => &mut self.bc.value,
                "de" => &mut self.de.value,
                "hl" => &mut self.hl.value,
                _ => panic!("Invalid register pair")
            }
        }
    }
}

struct Emulator {
    ram: RAM,
    reg: RegisterArray,
}

impl Emulator {
    fn new() -> Self {
        Emulator { ram: RAM::new(), reg: RegisterArray::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registerarray() {
        let mut regs = RegisterArray::new();

        regs["wz"] = 0xabcd;
        assert_eq!(regs["wz"], 0xabcd);
        assert_eq!(regs['w'], 0xab);

        regs['b'] = 0xcd;
        regs['c'] = 0xab;
        assert_eq!(regs["bc"], 0xcdab);
        assert_eq!(regs['b'], 0xcd);

        regs["wz"] = 0xffff;
        assert_eq!(regs["wz"], 0xffff);
        regs['z'] = 0xaa;
        assert_eq!(regs['z'], 0xaa);
        assert_eq!(regs["wz"], 0xffaa);
    }

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
