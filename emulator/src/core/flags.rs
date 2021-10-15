
pub struct Flags {
    /*
     * Represents processor flags
     * [zero, carry, sign, parity, aux, 0, 0, 0]
     */
    flags: u8,
}

impl Flags {
    pub fn new() -> Self {
        Flags { flags: 0 }
    }

    pub fn get(&self, flag: &str) -> bool {
        match flag {
            "zero" => (self.flags & 0x80) != 0,
            "carry" => (self.flags & 0x40) != 0,
            "sign" => (self.flags & 0x20) != 0,
            "parity" => (self.flags & 0x10) != 0,
            "aux" => (self.flags & 0x8) != 0,
            _ => panic!("Invalid flag"),
        }
    }

    pub fn set(&mut self, flag: &str) {
        match flag {
            "zero" => self.flags |= 0x80,
            "carry" => self.flags |= 0x40,
            "sign" => self.flags |= 0x20,
            "parity" => self.flags |= 0x10,
            "aux" => self.flags |= 0x8,
            _ => panic!("Invalid flag"),
        }
    }

    pub fn flip(&mut self, flag: &str) {
        match flag {
            "zero" => self.flags ^= 0x80,
            "carry" => self.flags ^= 0x40,
            "sign" => self.flags ^= 0x20,
            "parity" => self.flags ^= 0x10,
            "aux" => self.flags ^= 0x8,
            _ => panic!("Invalid flag"),
        }
    }
}

#[cfg(test)]
mod flags_tests {
    use super::*;

    #[test]
    fn test_flags() {
        let mut f = Flags::new();
        f.set("zero");
        f.set("parity");
        f.set("aux");
        assert!(f.get("zero"));
        assert!(!f.get("carry"));
        assert!(!f.get("sign"));
        assert!(f.get("parity"));
        assert!(f.get("aux"));

        f.flip("zero");
        assert!(!f.get("zero"));

        f.flip("carry");
        assert!(f.get("carry"));

        f.set("sign");
        assert!(f.get("sign"));

        f.set("parity");
        assert!(f.get("parity"));
    }
}
