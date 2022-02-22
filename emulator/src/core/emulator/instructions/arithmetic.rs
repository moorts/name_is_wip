use super::super::{Emulator, EResult};

impl Emulator {
    pub fn add(&mut self, opcode: u8, use_carry: bool) -> EResult<()> {
        // ADD
        let registers = ['b', 'c', 'd', 'e', 'h', 'l', 'm', 'a'];
        let mut index = (opcode & 0xF) as usize;
        if use_carry {
            index -= 8;
        }
        let register = registers[index];
        if register == 'm' {
            self.add_memory(use_carry)
        } else {
            self.add_register(register, use_carry)
        }
    }

    fn add_memory(&mut self, use_carry: bool) -> EResult<()> {
        let address = self.reg["hl"];
        let mut memoryValue = self.ram[address] as u16;
        if use_carry && self.reg.get_flag("carry") {
            memoryValue += 1;
        }
        self.add_value(memoryValue)
    }

    fn add_register(&mut self, register: char, use_carry: bool) -> EResult<()> {
        let mut register_value = self.reg[register] as u16;
        if use_carry && self.reg.get_flag("carry") {
            register_value += 1;
        }
        self.add_value(register_value)
    }

    fn add_value(&mut self, value: u16) -> EResult<()> {
        let accumulator = self.reg['a'] as u16;
        let result = accumulator + value;
        let resultByte = (result & 0xff) as u8;
        self.reg.set_flag("zero", (result & 0xff) == 0);
        self.reg.set_flag("sign", (result & 0x80) != 0);
        self.reg.set_flag("carry", result > 0xff);
        self.reg.set_flag("parity", resultByte.count_ones() & 1 == 0);
        self.reg.set_flag("aux", ((accumulator & 0x0F) + (value & 0x0F)) > 0x0F);
        self.reg['a'] = (result & 0xff) as u8;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_reg() {
        let mut e = Emulator::new();

        // ADD B, ADD A
        e.ram.load_vec(vec![0x80, 0x87], 0);

        e.reg['b'] = 69;
        e.reg['a'] = 42;

        e.execute_next().expect("Fuck");

        assert_eq!(e.reg['a'], 111);

        e.execute_next().expect("Fuck");

        assert_eq!(e.reg['a'], 222);
    }

    #[test]
    fn add_mem() {
        let mut e = Emulator::new();

        // ADD M with address 0x01
        e.ram.load_vec(vec![0x86, 69], 0);
        e.reg["hl"] = 0x01;

        e.reg['a'] = 42;

        e.execute_next().expect("Fuck");

        assert_eq!(e.reg['a'], 111);
    }

    #[test]
    fn add_flags() {
        let mut e = Emulator::new();

        // ADD B, ADD A, ADD A
        e.ram.load_vec(vec![0x80, 0x87, 0x87], 0);

        e.reg['b'] = 69;
        e.reg['a'] = 42;

        e.execute_next().expect("Fuck"); // Result is 111

        assert_eq!(e.reg.get_flag("carry"), false, "Carry bit");
        assert_eq!(e.reg.get_flag("sign"), false, "Sign bit");
        assert_eq!(e.reg.get_flag("zero"), false, "Zero bit");
        assert_eq!(e.reg.get_flag("parity"), true, "Parity bit");
        assert_eq!(e.reg.get_flag("aux"), false, "Auxiliary Carry bit");

        e.execute_next().expect("Fuck"); // Result is 222 -> sign is true

        assert_eq!(e.reg.get_flag("carry"), false, "Carry bit");
        assert_eq!(e.reg.get_flag("sign"), true, "Sign bit");
        assert_eq!(e.reg.get_flag("zero"), false, "Zero bit");
        assert_eq!(e.reg.get_flag("parity"), true, "Parity bit");
        assert_eq!(e.reg.get_flag("aux"), true, "Auxiliary Carry bit");

        e.execute_next().expect("Fuck"); // Result is 444 -> overflow to 188

        assert_eq!(e.reg.get_flag("carry"), true, "Carry bit");
        assert_eq!(e.reg.get_flag("sign"), true, "Sign bit");
        assert_eq!(e.reg.get_flag("zero"), false, "Zero bit");
        assert_eq!(e.reg.get_flag("parity"), false, "Parity bit");
        assert_eq!(e.reg.get_flag("aux"), true, "Auxiliary Carry bit");

        // Test auxiliary carry flag with example from manual
        // ADD B
        e.ram.load_vec(vec![0x80], 0);

        e.pc = 0;
        e.reg['b'] = 0x2E;
        e.reg['a'] = 0x74;

        e.execute_next().expect("Fuck");

        assert_eq!(e.reg['a'], 0xA2);
        assert_eq!(e.reg.get_flag("aux"), true);
    }

    #[test]
    fn adc() {
        let mut e = Emulator::new();

        // ADC B without carry
        e.ram.load_vec(vec![0x88], 0);

        e.reg['b'] = 69;
        e.reg['a'] = 42;
        e.reg.set_flag("carry", false);

        e.execute_next().expect("Fuck");

        assert_eq!(e.reg['a'], 111);

        // ADC B with carry
        e.ram.load_vec(vec![0x88], 0);
        e.pc = 0;

        e.reg['b'] = 69;
        e.reg['a'] = 42;
        e.reg.set_flag("carry", true);

        e.execute_next().expect("Fuck");

        assert_eq!(e.reg['a'], 112);
    }
}

