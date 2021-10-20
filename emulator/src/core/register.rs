use std::ops::{Index, IndexMut, Range};

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

pub struct RegisterArray {
    wz: Register,
    bc: Register, // Pair B (B and C)
    de: Register, // Pair D (D and E)
    hl: Register, // Pair H (H and L)
    psw: Register  // Pair PSW (Acc and Flags)
}

impl RegisterArray {
    pub fn new() -> Self {
        RegisterArray {
            wz: Register::new(),
            bc: Register::new(),
            de: Register::new(),
            hl: Register::new(),
            psw: Register::new()
        }
    }

    pub fn get_flag(&self, flag: &str) -> bool {
        unsafe {
            match flag {
                "zero" => (self.psw.bytes.1 & 0x80) != 0,
                "carry" => (self.psw.bytes.1 & 0x40) != 0,
                "sign" => (self.psw.bytes.1 & 0x20) != 0,
                "parity" => (self.psw.bytes.1 & 0x10) != 0,
                "aux" => (self.psw.bytes.1 & 0x8) != 0,
                _ => panic!("Invalid flag"),
            }
        }
    }

    pub fn set_flag(&mut self, flag: &str) {
        unsafe {
            match flag {
                "zero" => self.psw.bytes.1 |= 0x80,
                "carry" => self.psw.bytes.1 |= 0x40,
                "sign" => self.psw.bytes.1 |= 0x20,
                "parity" => self.psw.bytes.1 |= 0x10,
                "aux" => self.psw.bytes.1 |= 0x8,
                _ => panic!("Invalid flag"),
            }
        }
    }

    pub fn flip_flag(&mut self, flag: &str) {
        unsafe {
            match flag {
                "zero" => self.psw.bytes.1 ^= 0x80,
                "carry" => self.psw.bytes.1 ^= 0x40,
                "sign" => self.psw.bytes.1 ^= 0x20,
                "parity" => self.psw.bytes.1 ^= 0x10,
                "aux" => self.psw.bytes.1 ^= 0x8,
                _ => panic!("Invalid flag"),
            }
        }
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
                'a' => &self.psw.bytes.0,
                _ => panic!("Invalid register"),
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
                'a' => &mut self.psw.bytes.0,
                _ => panic!("Invalid register"),
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
                "psw" => &self.psw.value,
                _ => panic!("Invalid register pair"),
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
                "psw" => &mut self.psw.value,
                _ => panic!("Invalid register pair"),
            }
        }
    }
}

#[cfg(test)]
mod register_tests {
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
    fn test_flags() {
        let mut regs = RegisterArray::new();

        regs.set_flag("zero");
        regs.set_flag("parity");
        regs.set_flag("aux");
        assert!(regs.get_flag("zero"));
        assert!(!regs.get_flag("carry"));
        assert!(!regs.get_flag("sign"));
        assert!(regs.get_flag("parity"));
        assert!(regs.get_flag("aux"));

        regs.flip_flag("zero");
        assert!(!regs.get_flag("zero"));

        regs.flip_flag("carry");
        assert!(regs.get_flag("carry"));

        regs.set_flag("sign");
        assert!(regs.get_flag("sign"));

        regs.set_flag("parity");
        assert!(regs.get_flag("parity"));
    }

}
