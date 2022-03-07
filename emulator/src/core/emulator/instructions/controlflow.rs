use super::super::{EResult, Emulator};

const REGISTERS: [char; 8] = ['b', 'c', 'd', 'e', 'h', 'l', 'm', 'a'];

impl<'a> Emulator<'a> {
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
        let mut e = Emulator::new();

        e.sp = 0x3fff;
        e.ram[0x1234] = 0xc9;

        e.call(0x1234).expect("Fuck");
        assert_eq!(e.sp, 0x3fff - 2);
        assert_eq!(e.pc, 0x1234);

        e.execute_next().expect("Fuck");
        assert_eq!(e.pc, 0x0);
    }

    #[test]
    fn jmp_if() {
        let mut e = Emulator::new();

        e.ram.load_vec(vec![0x04, 0x00, 0x00, 0x00], 0);

        // Performs
        // a) one failing jmp (-> pc = 2)
        // b) one succeeding jmp (pc = ram[pc] = ram[2] -> 0)
        // c) Back in starting position
        // -> Repeat for each flag
        for flag in vec!["zero", "carry", "sign", "parity", "aux"] {
            e.jmp_if(flag).expect("");
            assert_eq!(e.pc, 2);
            e.reg.set_flag(flag, true);
            e.jmp_if(flag).expect("");
            assert_eq!(e.pc, 0);
        }
    }

    #[test]
    fn jmp_not() {
        let mut e = Emulator::new();

        e.ram.load_vec(vec![0x04, 0x00, 0x00, 0x00], 0);
        e.reg.set_flags(0xff);

        // same as tests::jmp_if
        for flag in vec!["zero", "carry", "sign", "parity", "aux"] {
            e.jmp_not(flag).expect("");
            assert_eq!(e.pc, 2);
            e.reg.flip_flag(flag);
            e.jmp_not(flag).expect("");
            assert_eq!(e.pc, 0);
        }
    }

    #[test]
    fn call_if() {
        let mut e = Emulator::new();

        e.sp = 0x3fff;

        e.ram.load_vec(vec![0x00, 0x00, 0x11, 0x11], 0);

        for flag in vec!["zero", "carry", "sign", "parity", "aux"] {
            e.call_if(flag).expect("");
            assert_eq!(e.pc, 2);
            e.ret_if(flag).expect("");
            assert_eq!(e.pc, 2);
            e.reg.set_flag(flag, true);
            e.call_if(flag).expect("");
            assert_eq!(e.pc, 0x1111);
            e.ret_if(flag).expect("");
            assert_eq!(e.pc, 4);
            e.pc = 0;
        }
    }

    #[test]
    fn call_not() {
        let mut e = Emulator::new();

        e.sp = 0x3fff;

        e.ram.load_vec(vec![0x00, 0x00, 0x11, 0x11], 0);
        e.reg.set_flags(0xff);

        for flag in vec!["zero", "carry", "sign", "parity", "aux"] {
            e.call_not(flag).expect("");
            assert_eq!(e.pc, 2);
            e.ret_not(flag).expect("");
            assert_eq!(e.pc, 2);
            e.reg.flip_flag(flag);
            e.call_not(flag).expect("");
            assert_eq!(e.pc, 0x1111);
            e.ret_not(flag).expect("");
            assert_eq!(e.pc, 4);
            e.pc = 0;
        }
    }

    #[test]
    fn call() {
        let mut e = Emulator::new();

        e.sp = 0x3fff;

        assert_eq!(e.pc, 0x0);
        e.call_if("carry").expect("Fuck");
        assert_eq!(e.pc, 0x2);
        e.reg.set_flag("carry", true);
        e.ram.load_vec(vec![0x34, 0x12], 2);
        e.call_if("carry").expect("Fuck");
        assert_eq!(e.pc, 0x1234);
    }

    #[test]
    fn rst() {
        let mut e = Emulator::new();

        e.pc = 0x1111;
        e.sp = 0x3fff;

        e.ram
            .load_vec(vec![0xc7, 0xcf, 0xd7, 0xdf, 0xe7, 0xef, 0xf7, 0xff], e.pc);
        for i in 0x1111..0x1119 {
            e.pc = i as u16;
            e.execute_next().expect("Fuck");
            assert_eq!(e.pc, (i - 0x1111) * 8);
        }
    }
}
