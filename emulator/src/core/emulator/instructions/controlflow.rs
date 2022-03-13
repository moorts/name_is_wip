use super::super::{Emulator, EResult};

const REGISTERS: [char; 8] = ['b', 'c', 'd', 'e', 'h', 'l', 'm', 'a'];

impl Emulator {
    pub fn jmp_not(&mut self, flag: &str) -> EResult<()> {
        if !self.reg.get_flag(flag) {
            self.pc = self.read_addr()?;
        } else {
            self.pc += 2;
        }
        Ok(())
    }

    pub fn jmp_if(&mut self, flag: &str) -> EResult<()> {
        if self.reg.get_flag(flag) {
            self.pc = self.read_addr()?;
        } else {
            self.pc += 2;
        }
        Ok(())
    }

    pub fn call_not(&mut self, flag: &str) -> EResult<()> {
        if !self.reg.get_flag(flag) {
            self.call_imm()?;
        } else {
            self.pc += 2;
        }
        Ok(())
    }

    pub fn call_if(&mut self, flag: &str) -> EResult<()> {
        if self.reg.get_flag(flag) {
            self.call_imm()?;
        } else {
            self.pc += 2;
        }
        Ok(())
    }

    pub fn call_imm(&mut self) -> EResult<()> {
        let adr = self.read_addr()?;
        self.push(self.pc)?;
        self.pc = adr;
        Ok(())
    }

    pub fn call(&mut self, adr: u16) -> EResult<()> {
        self.push(self.pc)?;
        self.pc = adr;
        Ok(())
    }

    pub fn ret_if(&mut self, flag: &str) -> EResult<()> {
        if self.reg.get_flag(flag) {
            self.ret()?;
        }
        Ok(())
    }

    pub fn ret_not(&mut self, flag: &str) -> EResult<()> {
        if !self.reg.get_flag(flag) {
            self.ret()?;
        }
        Ok(())
    }

    pub fn ret(&mut self) -> EResult<()> {
        self.pc = self.pop()?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    
    #[test]
    fn call_ret() {
        let mut emu = Emulator::new();

        emu.sp = 0x3fff;
        emu.ram[0x1234] = 0xc9;

        emu.call(0x1234).expect("Fuck");
        assert_eq!(emu.sp, 0x3fff - 2);
        assert_eq!(emu.pc, 0x1234);

        emu.execute_next().expect("Fuck");
        assert_eq!(emu.pc, 0x0);
    }

    #[test]
    fn jmp_if() {
        let mut emu = Emulator::new();

        emu.ram.load_vec(vec![0x04, 0x00, 0x00, 0x00], 0);

        // Performs
        // a) one failing jmp (-> pc = 2)
        // b) one succeeding jmp (pc = ram[pc] = ram[2] -> 0)
        // c) Back in starting position
        // -> Repeat for each flag
        for flag in vec!["zero", "carry", "sign", "parity", "aux"] {
            emu.jmp_if(flag).expect("");
            assert_eq!(emu.pc, 2);
            emu.reg.set_flag(flag, true);
            emu.jmp_if(flag).expect("");
            assert_eq!(emu.pc, 0);
        }
    }

    #[test]
    fn jmp_not() {
        let mut emu = Emulator::new();

        emu.ram.load_vec(vec![0x04, 0x00, 0x00, 0x00], 0);
        emu.reg.set_flags(0xff);

        // same as tests::jmp_if
        for flag in vec!["zero", "carry", "sign", "parity", "aux"] {
            emu.jmp_not(flag).expect("");
            assert_eq!(emu.pc, 2);
            emu.reg.flip_flag(flag);
            emu.jmp_not(flag).expect("");
            assert_eq!(emu.pc, 0);
        }
    }

    #[test]
    fn call_if() {
        let mut emu = Emulator::new();

        emu.sp = 0x3fff;

        emu.ram.load_vec(vec![0x00, 0x00, 0x11, 0x11], 0);

        for flag in vec!["zero", "carry", "sign", "parity", "aux"] {
            emu.call_if(flag).expect("");
            assert_eq!(emu.pc, 2);
            emu.ret_if(flag).expect("");
            assert_eq!(emu.pc, 2);
            emu.reg.set_flag(flag, true);
            emu.call_if(flag).expect("");
            assert_eq!(emu.pc, 0x1111);
            emu.ret_if(flag).expect("");
            assert_eq!(emu.pc, 4);
            emu.pc = 0;
        }
    }

    #[test]
    fn call_not() {
        let mut emu = Emulator::new();

        emu.sp = 0x3fff;

        emu.ram.load_vec(vec![0x00, 0x00, 0x11, 0x11], 0);
        emu.reg.set_flags(0xff);

        for flag in vec!["zero", "carry", "sign", "parity", "aux"] {
            emu.call_not(flag).expect("");
            assert_eq!(emu.pc, 2);
            emu.ret_not(flag).expect("");
            assert_eq!(emu.pc, 2);
            emu.reg.flip_flag(flag);
            emu.call_not(flag).expect("");
            assert_eq!(emu.pc, 0x1111);
            emu.ret_not(flag).expect("");
            assert_eq!(emu.pc, 4);
            emu.pc = 0;
        }
    }

    #[test]
    fn call() {
        let mut emu = Emulator::new();

        emu.sp = 0x3fff;

        assert_eq!(emu.pc, 0x0);
        emu.call_if("carry").expect("Fuck");
        assert_eq!(emu.pc, 0x2);
        emu.reg.set_flag("carry", true);
        emu.ram.load_vec(vec![0x34, 0x12], 2);
        emu.call_if("carry").expect("Fuck");
        assert_eq!(emu.pc, 0x1234);
    }

    #[test]
    fn rst() {
        let mut emu = Emulator::new();

        emu.pc = 0x1111;
        emu.sp = 0x3fff;

        emu.ram
            .load_vec(vec![0xc7, 0xcf, 0xd7, 0xdf, 0xe7, 0xef, 0xf7, 0xff], emu.pc);
        for i in 0x1111..0x1119 {
            emu.pc = i as u16;
            emu.execute_next().expect("Fuck");
            assert_eq!(emu.pc, (i - 0x1111) * 8);
        }
    }
}

