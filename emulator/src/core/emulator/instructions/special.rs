use super::super::{Emulator};

impl Emulator {
    pub fn daa(&mut self) {
        let mut acc = self.reg['a'];
        let mut low = acc & 0x0F;
        if low > 9 || self.reg.get_flag("aux") {
            acc = acc.wrapping_add(6);
            low = low + 6;
        }
        let mut high = (acc & 0xF0) >> 4;
        if high > 9 || self.reg.get_flag("carry") {
            high += 6;
        }
        let result = ((high & 0x0F) << 4) + (low & 0x0F);
        self.reg.set_flag("aux", low > 0x0F);
        self.reg.set_flag("carry", high > 0x0F);
        self.reg.set_flag("zero", result == 0);
        self.reg.set_flag("sign", (result & 0x80) != 0);
        self.reg.set_flag("parity", result.count_ones() & 1 == 0);
        self.reg['a'] = result;
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

