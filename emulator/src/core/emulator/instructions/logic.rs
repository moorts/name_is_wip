use crate::core::emulator::{EResult, Emulator};

const REGISTERS: [char; 8] = ['b', 'c', 'd', 'e', 'h', 'l', 'm', 'a'];

impl Emulator {
    pub fn and(&mut self, opcode: u8) -> EResult<()> {
        let mut index = (opcode & 0xF) as usize;
        let register = REGISTERS[index];
        if register == 'm' {
            let address = self.reg["hl"];
            self.and_value(self.ram[address])
        } else {
            self.and_value(self.reg[register])
        }
    }
    
    fn and_value(&mut self, value: u8) -> EResult<()> {
        let accumulator = self.reg['a'];
        let result = accumulator & value;
        self.reg.set_flag("zero", (result & 0xff) == 0);
        self.reg.set_flag("sign", (result & 0x80) != 0);
        self.reg.set_flag("carry", false);
        self.reg.set_flag("parity", result.count_ones() & 1 == 0);
        self.reg.set_flag("aux", false);
        self.reg['a'] = result;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn and() {
        let mut e = Emulator::new();

        // ANA B, ANA M
        e.ram.load_vec(vec![0xA0, 0xA6], 0);

        e.reg['b'] = 0b1111_1100;
        e.reg['a'] = 0b0000_1111;
        e.reg["hl"] = 0x01;

        e.execute_next().expect("Fuck");

        assert_eq!(e.reg['a'], 0b0000_1100);
        assert_eq!(e.reg.get_flag("carry"), false, "Carry bit");
        assert_eq!(e.reg.get_flag("sign"), false, "Sign bit");
        assert_eq!(e.reg.get_flag("zero"), false, "Zero bit");
        assert_eq!(e.reg.get_flag("parity"), true, "Parity bit");
        assert_eq!(e.reg.get_flag("aux"), false, "Auxiliary Carry bit");
        
        e.execute_next().expect("Fuck");
        
        assert_eq!(e.reg['a'], 0b0000_0100);
    }
}