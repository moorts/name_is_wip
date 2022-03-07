use crate::core::emulator::{EResult, Emulator};

const REGISTERS: [char; 8] = ['b', 'c', 'd', 'e', 'h', 'l', 'm', 'a'];

impl Emulator {
    pub fn and(&mut self, opcode: u8) -> EResult<()> {
        let index = (opcode & 0xF) as usize;
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
        self.set_flags(result);
        self.reg['a'] = result;
        Ok(())
    }

    pub fn xor(&mut self, opcode: u8) -> EResult<()> {
        let index = ((opcode - 8) & 0xF) as usize;
        let register = REGISTERS[index];
        if register == 'm' {
            let address = self.reg["hl"];
            self.xor_value(self.ram[address])
        } else {
            self.xor_value(self.reg[register])
        }
    }

    fn xor_value(&mut self, value: u8) -> EResult<()> {
        let accumulator = self.reg['a'];
        let result = accumulator ^ value;
        self.set_flags(result);
        self.reg['a'] = result;
        Ok(())
    }

    pub fn or(&mut self, opcode: u8) -> EResult<()> {
        let index = (opcode & 0xF) as usize;
        let register = REGISTERS[index];
        if register == 'm' {
            let address = self.reg["hl"];
            self.or_value(self.ram[address])
        } else {
            self.or_value(self.reg[register])
        }
    }

    fn or_value(&mut self, value: u8) -> EResult<()> {
        let accumulator = self.reg['a'];
        let result = accumulator | value;
        self.set_flags(result);
        self.reg['a'] = result;
        Ok(())
    }

    pub fn cmp(&mut self, opcode: u8) -> EResult<()> {
        let index = ((opcode - 8) & 0xF) as usize;
        let register = REGISTERS[index];
        if register == 'm' {
            let address = self.reg["hl"];
            self.cmp_value(self.ram[address])
        } else {
            self.cmp_value(self.reg[register])
        }
    }

    fn cmp_value(&mut self, value: u8) -> EResult<()> {
        // Perform SUB but restore accumulator afterwards
        let accumulator = self.reg['a'];
        self.sub_value(value as u16)?;
        self.reg['a'] = accumulator;
        Ok(())
    }

    fn set_flags(&mut self, result: u8) {
        self.reg.set_flag("zero", (result & 0xff) == 0);
        self.reg.set_flag("sign", (result & 0x80) != 0);
        self.reg.set_flag("carry", false);
        self.reg.set_flag("parity", result.count_ones() & 1 == 0);
        self.reg.set_flag("aux", false);
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

    #[test]
    fn xor() {
        let mut e = Emulator::new();

        // XRA B
        e.ram.load_vec(vec![0xA8], 0);

        e.reg['b'] = 0b1111_1100;
        e.reg['a'] = 0b0000_1111;

        e.execute_next().expect("Fuck");

        assert_eq!(e.reg['a'], 0b1111_0011);
        assert_eq!(e.reg.get_flag("carry"), false, "Carry bit");
        assert_eq!(e.reg.get_flag("sign"), true, "Sign bit");
        assert_eq!(e.reg.get_flag("zero"), false, "Zero bit");
        assert_eq!(e.reg.get_flag("parity"), true, "Parity bit");
        assert_eq!(e.reg.get_flag("aux"), false, "Auxiliary Carry bit");
    }

    #[test]
    fn or() {
        let mut e = Emulator::new();

        // ORA B
        e.ram.load_vec(vec![0xB0], 0);

        e.reg['b'] = 0b1111_1100;
        e.reg['a'] = 0b0000_1110;

        e.execute_next().expect("Fuck");

        assert_eq!(e.reg['a'], 0b1111_1110);
        assert_eq!(e.reg.get_flag("carry"), false, "Carry bit");
        assert_eq!(e.reg.get_flag("sign"), true, "Sign bit");
        assert_eq!(e.reg.get_flag("zero"), false, "Zero bit");
        assert_eq!(e.reg.get_flag("parity"), false, "Parity bit");
        assert_eq!(e.reg.get_flag("aux"), false, "Auxiliary Carry bit");
    }

    #[test]
    fn cmp() {
        let mut e = Emulator::new();

        // CMP B
        e.ram.load_vec(vec![0xB8], 0);

        e.reg['b'] = 0x05;
        e.reg['a'] = 0x0A;

        e.execute_next().expect("Fuck");

        assert_eq!(e.reg['a'], 0x0A);
        assert_eq!(e.reg.get_flag("carry"), false, "Carry bit");
        assert_eq!(e.reg.get_flag("zero"), false, "Zero bit");

        e.pc = 0;
        e.reg['b'] = 0x05;
        e.reg['a'] = 0x02;

        e.execute_next().expect("Fuck");

        assert_eq!(e.reg.get_flag("carry"), true, "Carry bit");
        assert_eq!(e.reg.get_flag("zero"), false, "Zero bit");

        e.pc = 0;
        e.reg['b'] = 0x05;
        e.reg['a'] = 0xE5;

        e.execute_next().expect("Fuck");

        assert_eq!(e.reg.get_flag("carry"), false, "Carry bit");
        assert_eq!(e.reg.get_flag("zero"), false, "Zero bit");
    }
}
