use crate::core::emulator::{Emulator};

const REGISTERS: [char; 8] = ['b', 'c', 'd', 'e', 'h', 'l', 'm', 'a'];

impl Emulator {
    pub fn and(&mut self, opcode: u8) {
        let index = (opcode & 0xF) as usize;
        let register = REGISTERS[index];
        if register == 'm' {
            let address = self.reg["hl"];
            self.and_value(self.ram[address]);
        } else {
            self.and_value(self.reg[register]);
        }
    }
    
    pub fn and_value(&mut self, value: u8) {
        let accumulator = self.reg['a'];
        let result = accumulator & value;
        self.set_flags(result);
        self.reg.set_flag("aux", ((accumulator | value) & 0x08) != 0);
        self.reg['a'] = result;
    }
    
    pub fn xor(&mut self, opcode: u8) {
        let index = ((opcode - 8) & 0xF) as usize;
        let register = REGISTERS[index];
        if register == 'm' {
            let address = self.reg["hl"];
            self.xor_value(self.ram[address]);
        } else {
            self.xor_value(self.reg[register]);
        }
    }
    
    pub fn xor_value(&mut self, value: u8) {
        let accumulator = self.reg['a'];
        let result = accumulator ^ value;
        self.set_flags(result);
        self.reg['a'] = result;
    }
    
    pub fn or(&mut self, opcode: u8) {
        let index = (opcode & 0xF) as usize;
        let register = REGISTERS[index];
        if register == 'm' {
            let address = self.reg["hl"];
            self.or_value(self.ram[address]);
        } else {
            self.or_value(self.reg[register]);
        }
    }
    
    pub fn or_value(&mut self, value: u8) {
        let accumulator = self.reg['a'];
        let result = accumulator | value;
        self.set_flags(result);
        self.reg['a'] = result;
    }
    
    pub fn cmp(&mut self, opcode: u8) {
        let index = ((opcode - 8) & 0xF) as usize;
        let register = REGISTERS[index];
        if register == 'm' {
            let address = self.reg["hl"];
            self.cmp_value(self.ram[address]);
        } else {
            self.cmp_value(self.reg[register]);
        }
    }
    
    pub fn cmp_value(&mut self, value: u8) {
        // Perform SUB but restore accumulator afterwards
        let accumulator = self.reg['a'];
        self.sub_value(value, false);
        self.reg['a'] = accumulator; 
    }
    
    fn set_flags(&mut self, result: u8) {
        self.reg.set_flag("zero", (result & 0xff) == 0);
        self.reg.set_flag("sign", (result & 0x80) != 0);
        self.reg.set_flag("carry", false);
        self.reg.set_flag("parity", result.count_ones() & 1 == 0);
        self.reg.set_flag("aux", false);
    }
    
    pub fn rlc(&mut self) {
        let acc = self.reg['a'];
        self.reg.set_flag("carry", (acc & 0x80) != 0);
        self.reg['a'] = acc.rotate_left(1);
    }
    
    pub fn rrc(&mut self) {
        let acc = self.reg['a'];
        self.reg.set_flag("carry", (acc & 0x01) != 0);
        self.reg['a'] = acc.rotate_right(1);
    }
    
    pub fn ral(&mut self) {
        let acc = self.reg['a'];
        let carry = self.reg.get_flag("carry");
        self.reg.set_flag("carry", (acc & 0x80) != 0);
        self.reg['a'] = acc << 1;
        if carry {
            self.reg['a'] |= 0x01;
        } else {
            self.reg['a'] &= !0x01;
        }
    }
    
    pub fn rar(&mut self) {
        let acc = self.reg['a'];
        let carry = self.reg.get_flag("carry");
        self.reg.set_flag("carry", (acc & 0x01) != 0);
        self.reg['a'] = acc >> 1;
        if carry {
            self.reg['a'] |= 0x80;
        } else {
            self.reg['a'] &= !0x80;
        }
    }
    
    pub fn cma(&mut self) {
        self.reg['a'] = !self.reg['a'];
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn and() {
        let mut emu = Emulator::new();

        // ANA B, ANA M
        emu.ram.load_vec(vec![0xA0, 0xA6], 0);
        emu.reg['b'] = 0b1111_1100;
        emu.reg['a'] = 0b0000_1111;
        emu.reg["hl"] = 0x01;
        emu.execute_next().expect("Fuck");

        assert_eq!(emu.reg['a'], 0b0000_1100);
        assert_eq!(emu.reg.get_flag("carry"), false, "Carry bit");
        assert_eq!(emu.reg.get_flag("sign"), false, "Sign bit");
        assert_eq!(emu.reg.get_flag("zero"), false, "Zero bit");
        assert_eq!(emu.reg.get_flag("parity"), true, "Parity bit");
        assert_eq!(emu.reg.get_flag("aux"), true, "Auxiliary Carry bit");
        
        emu.execute_next().expect("Fuck");
        
        assert_eq!(emu.reg['a'], 0b0000_0100);
    }
    
    #[test]
    fn xor() {
        let mut emu = Emulator::new();

        // XRA B
        emu.ram.load_vec(vec![0xA8], 0);
        emu.reg['b'] = 0b1111_1100;
        emu.reg['a'] = 0b0000_1111;
        emu.execute_next().expect("Fuck");

        assert_eq!(emu.reg['a'], 0b1111_0011);
        assert_eq!(emu.reg.get_flag("carry"), false, "Carry bit");
        assert_eq!(emu.reg.get_flag("sign"), true, "Sign bit");
        assert_eq!(emu.reg.get_flag("zero"), false, "Zero bit");
        assert_eq!(emu.reg.get_flag("parity"), true, "Parity bit");
        assert_eq!(emu.reg.get_flag("aux"), false, "Auxiliary Carry bit");
    }
    
    #[test]
    fn or() {
        let mut emu = Emulator::new();

        // ORA B
        emu.ram.load_vec(vec![0xB0], 0);
        emu.reg['b'] = 0b1111_1100;
        emu.reg['a'] = 0b0000_1110;
        emu.execute_next().expect("Fuck");

        assert_eq!(emu.reg['a'], 0b1111_1110);
        assert_eq!(emu.reg.get_flag("carry"), false, "Carry bit");
        assert_eq!(emu.reg.get_flag("sign"), true, "Sign bit");
        assert_eq!(emu.reg.get_flag("zero"), false, "Zero bit");
        assert_eq!(emu.reg.get_flag("parity"), false, "Parity bit");
        assert_eq!(emu.reg.get_flag("aux"), false, "Auxiliary Carry bit");
    }
    
    #[test]
    fn cmp() {
        let mut emu = Emulator::new();

        // CMP B
        emu.ram.load_vec(vec![0xB8], 0);
        emu.reg['b'] = 0x05;
        emu.reg['a'] = 0x0A;
        emu.execute_next().expect("Fuck");

        assert_eq!(emu.reg['a'], 0x0A);
        assert_eq!(emu.reg.get_flag("carry"), false, "Carry bit");
        assert_eq!(emu.reg.get_flag("zero"), false, "Zero bit");
        
        emu.pc = 0;
        emu.reg['b'] = 0x05;
        emu.reg['a'] = 0x02;
        emu.execute_next().expect("Fuck");

        assert_eq!(emu.reg.get_flag("carry"), true, "Carry bit");
        assert_eq!(emu.reg.get_flag("zero"), false, "Zero bit");
        
        emu.pc = 0;
        emu.reg['b'] = 0x05;
        emu.reg['a'] = 0xE5;
        emu.execute_next().expect("Fuck");

        assert_eq!(emu.reg.get_flag("carry"), false, "Carry bit");
        assert_eq!(emu.reg.get_flag("zero"), false, "Zero bit");
    }
    
    #[test]
    fn rlc() {
        let mut emu = Emulator::new();

        // RLC
        emu.ram.load_vec(vec![0x07], 0);
        emu.reg['a'] = 0b1111_0000;
        emu.execute_next().expect("Fuck");

        assert_eq!(emu.reg['a'], 0b1110_0001);
        assert_eq!(emu.reg.get_flag("carry"), true, "Carry bit");
    }
    
    #[test]
    fn rrc() {
        let mut emu = Emulator::new();

        // RRC
        emu.ram.load_vec(vec![0x0F], 0);
        emu.reg['a'] = 0b0000_1111;
        emu.execute_next().expect("Fuck");

        assert_eq!(emu.reg['a'], 0b1000_0111);
        assert_eq!(emu.reg.get_flag("carry"), true, "Carry bit");
    }
    
    #[test]
    fn ral() {
        let mut emu = Emulator::new();

        // RAL
        emu.ram.load_vec(vec![0x17], 0);
        emu.reg['a'] = 0b1011_0101;
        emu.reg.set_flag("carry", false);
        emu.execute_next().expect("Fuck");

        assert_eq!(emu.reg['a'], 0b0110_1010);
        assert_eq!(emu.reg.get_flag("carry"), true, "Carry bit");
    }
    
    #[test]
    fn rar() {
        let mut emu = Emulator::new();

        // RAR
        emu.ram.load_vec(vec![0x1F], 0);
        emu.reg['a'] = 0b1011_0101;
        emu.reg.set_flag("carry", false);
        emu.execute_next().expect("Fuck");

        assert_eq!(emu.reg['a'], 0b0101_1010);
        assert_eq!(emu.reg.get_flag("carry"), true, "Carry bit");
    }
    
    #[test]
    fn cma() {
        let mut emu = Emulator::new();

        // CMA
        emu.ram.load_vec(vec![0x2F], 0);
        emu.reg['a'] = 0b1011_0101;
        emu.execute_next().expect("Fuck");

        assert_eq!(emu.reg['a'], 0b0100_1010);
    }
}