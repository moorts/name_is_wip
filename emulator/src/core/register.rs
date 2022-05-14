use std::ops::{Index, IndexMut};
use wasm_bindgen::prelude::wasm_bindgen;

#[repr(C)]
#[derive(Clone, Copy)]
union Register {
    bytes: (u8, u8),
    value: u16,
}

impl Register {
    fn new() -> Self {
        Register { value: 0 }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct RegisterArray {
    wz: Register,
    bc: Register, // Pair B (B and C)
    de: Register, // Pair D (D and E)
    hl: Register, // Pair H (H and L)
    psw: Register  // Pair PSW (Acc and Flags)
}

impl RegisterArray {
    pub fn new() -> Self {
        let mut new = RegisterArray {
            wz: Register::new(),
            bc: Register::new(),
            de: Register::new(),
            hl: Register::new(),
            psw: Register::new()
        };

        // that bit should always be 1, don't ask me why
        unsafe {
            new.psw.bytes.0 |= 0b0000_0010;
        }

        new
    }

    pub fn get_flag(&self, flag: &str) -> bool {
        unsafe {
            match flag {
                "zero" => (self.psw.bytes.0 & 0b0100_0000) != 0,
                "carry" => (self.psw.bytes.0 & 0b0000_0001) != 0,
                "sign" => (self.psw.bytes.0 & 0b1000_0000) != 0,
                "parity" => (self.psw.bytes.0 & 0b0000_0100) != 0,
                "aux" => (self.psw.bytes.0 & 0b0001_0000) != 0,
                _ => panic!("Invalid flag"),
            }
        }
    }

    pub fn set_flag(&mut self, flag: &str, value: bool) {
        unsafe {
            if value {
                match flag {
                    "zero" => self.psw.bytes.0 |= 0b0100_0000,
                    "carry" => self.psw.bytes.0 |= 0b0000_0001,
                    "sign" => self.psw.bytes.0 |= 0b1000_0000,
                    "parity" => self.psw.bytes.0 |= 0b0000_0100,
                    "aux" => self.psw.bytes.0 |= 0b0001_0000,
                    _ => panic!("Invalid flag"),
                }
            } else {
                match flag {
                    "zero" => self.psw.bytes.0 &= !0b0100_0000,
                    "carry" => self.psw.bytes.0 &= !0b0000_0001,
                    "sign" => self.psw.bytes.0 &= !0b1000_0000,
                    "parity" => self.psw.bytes.0 &= !0b0000_0100,
                    "aux" => self.psw.bytes.0 &= !0b0001_0000,
                    _ => panic!("Invalid flag"),
                }
            }
        }
    }

    pub fn flip_flag(&mut self, flag: &str) {
        unsafe {
            match flag {
                "zero" => self.psw.bytes.0 ^= 0b0100_0000,
                "carry" => self.psw.bytes.0 ^= 0b0000_0001,
                "sign" => self.psw.bytes.0 ^= 0b1000_0000,
                "parity" => self.psw.bytes.0 ^= 0b0000_0100,
                "aux" => self.psw.bytes.0 ^= 0b0001_0000,
                _ => panic!("Invalid flag"),
            }
        }
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.psw.bytes.0 = flags;
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
                'h' => &self.hl.bytes.1,
                'l' => &self.hl.bytes.0,
                'a' => &self.psw.bytes.1,
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
                'h' => &mut self.hl.bytes.1,
                'l' => &mut self.hl.bytes.0,
                'a' => &mut self.psw.bytes.1,
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
mod tests {
    use super::*;

    #[test]
    fn registerarray() {
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
    fn flags() {
        let mut regs = RegisterArray::new();

        regs.set_flag("zero", true);
        regs.set_flag("parity", true);
        regs.set_flag("aux", true);
        assert!(regs.get_flag("zero"));
        assert!(!regs.get_flag("carry"));
        assert!(!regs.get_flag("sign"));
        assert!(regs.get_flag("parity"));
        assert!(regs.get_flag("aux"));

        regs.flip_flag("zero");
        assert!(!regs.get_flag("zero"));

        regs.flip_flag("carry");
        assert!(regs.get_flag("carry"));

        regs.set_flag("sign", true);
        assert!(regs.get_flag("sign"));

        regs.set_flag("parity", true);
        assert!(regs.get_flag("parity"));
        
        regs.set_flag("parity", false);
        assert!(!regs.get_flag("parity"));
    }

}
