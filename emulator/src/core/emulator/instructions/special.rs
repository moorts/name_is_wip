use super::super::{Emulator};

impl Emulator {
    pub fn daa(&mut self) {
        let mut a: u8 = 0;
        let mut c = self.reg.get_flag("carry");
        let lsb = self.reg['a'] & 0x0F;
        let msb = self.reg['a'] >> 4;
        if lsb > 9 || self.reg.get_flag("aux") {
            a += 0x06;
        }
        if msb > 9 || self.reg.get_flag("carry") || (msb >= 9 && lsb > 9) {
            a += 0x60;
            c = true;
        }
        self.add_value(a as u16, false);
        self.reg.set_flag("carry", c);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn daa() {
        let mut emu = Emulator::new();
        
        // DAA
        emu.ram.load_vec(vec![0x27], 0);
        emu.reg['a'] = 0x9B;
        emu.execute_next().expect("Fuck");

        assert_eq!(emu.reg['a'], 1);
        assert_eq!(emu.reg.get_flag("carry"), true, "Carry bit");
        assert_eq!(emu.reg.get_flag("sign"), false, "Sign bit");
        assert_eq!(emu.reg.get_flag("zero"), false, "Zero bit");
        assert_eq!(emu.reg.get_flag("parity"), false, "Parity bit");
        assert_eq!(emu.reg.get_flag("aux"), true, "Auxiliary Carry bit");
    }
}

