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
                /*
                if !self.reg.get_flag("zero") {
                    self.pc = self.read_addr();
                } else {
                    self.pc += 2;
                }
                */
                self.jmp_not("zero");
            }
            0xc3 => {
                // JMP adr
                self.pc = self.read_addr();
            }
            0xc4 => unimplemented!(),
            0xc5 => {
                // PUSH B
                self.push(self.reg["bc"]);
            }
            0xc6 => unimplemented!(),
            0xc7 => {
                // RST 0
                self.call(0x0);
            }
            0xc8 => {
                // RZ
                if self.reg.get_flag("zero") {
                    self.pc = self.pop();
                }
            }
            0xc9 => {
                // RET
                self.pc = self.pop();
            }
            0xca => {
                // JZ adr
                /*
                if self.reg.get_flag("zero") {
                    self.pc = self.read_addr();
                } else {
                    self.pc += 2;
                }
                */
                self.jmp_if("zero");
                
            }
            0xcc => {
                // CZ addr
                if self.reg.get_flag("zero") {
                    let adr = self.read_addr();
                    self.call(adr);
                } else {
                    self.pc += 2;
                }
            }
            0xcd => {
                // CALL addr
                let adr = self.read_addr();
                self.call(adr);
            }
            0xce => {
                unimplemented!()
            }
            0xcf => {
                // RST 1
                self.call(0x8);
            }
            0xd0 => {
                // RNC
                if !self.reg.get_flag("carry") {
                    self.pc = self.pop();
                }
            }
            0xd1 => {
                // POP D
                self.reg["de"] = self.pop();
            }
            0xd2 => {
                // JNC adr
                if !self.reg.get_flag("carry") {
                    self.pc = self.read_addr();
                } else {
                    self.pc += 2;
                }
            }
            0xd3 => {
                // OUT
                unimplemented!()
            }
            0xd4 => {
                // CNC adr
                if !self.reg.get_flag("carry") {
                    let adr = self.read_addr();
                    self.call(adr);
                } else {
                    self.pc += 2;
                }

            }
            _ => unimplemented!("Opcode not yet implemented")
        }
        Ok(())
    }

    fn jmp_not(&mut self, flag: &str) {
        if !self.reg.get_flag(flag) {
            self.pc = self.read_addr();
        } else {
            self.pc += 2;
        }
    }

    fn jmp_if(&mut self, flag: &str) {
        if self.reg.get_flag(flag) {
            self.pc = self.read_addr();
        } else {
            self.pc += 2;
        }
    }

    fn call(&mut self, adr: u16) {
        self.push(self.pc);
        self.pc = adr;
    }

    fn push(&mut self, val: u16) {
        self.sp -= 1;
        self.ram[self.sp] = (val >> 8) as u8;
        self.sp -= 1;
        self.ram[self.sp] = val as u8;
    }

    fn pop(&mut self) -> u16 {
        let low = self.ram[self.sp] as u16;
        self.sp += 1;
        let high = self.ram[self.sp] as u16;
        self.sp += 1;
        (high << 8) | low

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
mod emulator_tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut e = Emulator::new();

        e.sp = 0x3fff;
        e.push(0xabcd);
        assert_eq!(e.sp, 0x3ffd);
        assert_eq!(0xabcd, e.pop());
        assert_eq!(e.sp, 0x3fff);
    }

    #[test]
    fn test_call_ret() {
        let mut e = Emulator::new();

        e.sp = 0x3fff;
        e.ram[0x1234] = 0xc9;

        e.call(0x1234);
        assert_eq!(e.sp, 0x3fff - 2);
        assert_eq!(e.pc, 0x1234);

        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x0);
    }

    #[test]
    fn test_jmps() {
        let mut e = Emulator::new();

        // Test JMP
        e.ram.load_vec(vec![0xc3, 0xcd, 0xab], 0);
        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0xabcd);

        // Test JZ
        e.ram.load_vec(vec![0xca, 0x03, 0x00, 0xca, 0x03, 0x00], 0xabcd);
        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0xabd0);
        e.reg.set_flag("zero");
        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x0003);

        // Test JNZ
        e.ram.load_vec(vec![0xc2, 0x34, 0x12, 0xc2, 0x34, 0x12], 0x3);
        e.reg.set_flag("zero");
        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x6);
        e.reg.flip_flag("zero");
        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x1234);
    }
}
