use crate::core::ram::RAM;
use crate::core::register::RegisterArray;
use crate::core::flags::Flags;



pub struct Emulator {
    pc: u16,
    sp: u16,
    acc: u8,
    ram: RAM,
    reg: RegisterArray,
    flags: Flags,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            pc: 0,
            sp: 0,
            acc: 0,
            ram: RAM::new(),
            reg: RegisterArray::new(),
            flags: Flags::new(),
        }
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
