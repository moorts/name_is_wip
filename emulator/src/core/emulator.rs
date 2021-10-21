use crate::core::ram::*;
use crate::core::register::RegisterArray;


pub struct Emulator {
    pc: u16,
    sp: u16,
    ram: Box<dyn RAM>,
    reg: RegisterArray,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            pc: 0,
            sp: 0,
            ram: Box::new(DefaultRam::new()),
            reg: RegisterArray::new(),
        }
    }

    pub fn execute_next(&mut self) -> Result<(), &'static str> {
        let opcode = self.ram[self.pc];
        self.pc += 1;
        match opcode {
            0xc2 => {
                // JNZ adr
                if !self.reg.get_flag("zero") {
                    self.pc = self.read_addr();
                }
            }
            0xc3 => {
                // JMP adr
                self.pc = self.read_addr();
            }
            0xc4 => unimplemented!(),
            0xc5 => unimplemented!(),
            0xc6 => unimplemented!(),
            0xc7 => unimplemented!(),
            0xc8 => unimplemented!(),
            0xc9 => unimplemented!(),
            0xca => {
                // JZ adr
                if self.reg.get_flag("zero") {
                    self.pc = self.read_addr();
                }
            }
            _ => unimplemented!("Opcode not yet implemented")
        }
        Ok(())
    }

    fn push(&mut self, val: u16) {
        self.sp += 1;
        self.ram[self.sp] = (val >> 8) as u8;
        self.sp += 1;
        self.ram[self.sp] = val as u8;
    }

    fn read_addr(&mut self) -> u16 {
        let low = self.ram[self.pc] as u16;
        self.pc += 1;
        let high = self.ram[self.pc] as u16;
        self.pc += 1;
        (high << 8) | low
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jmps() -> Result<(), &'static str> {
        let mut e = Emulator::new();

        // Test JMP
        e.ram[0] = 0xc3;
        e.ram[1] = 0xcd;
        e.ram[2] = 0xab;
        e.execute_next()?;
        assert_eq!(e.pc, 0xabcd);
        Ok(())
    }
}
