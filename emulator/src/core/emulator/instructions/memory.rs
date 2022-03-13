use super::super::{Emulator, EResult};

impl Emulator {
    pub fn stax(&mut self, register: &str) {
        self.ram[self.reg[register]] = self.reg['a'];
    }
    
    pub fn ldax(&mut self, register: &str) {
        self.reg['a'] = self.ram[self.reg[register]];
    }
    
    pub fn push(&mut self, val: u16) -> EResult<()> {
        if self.sp < 2 {
            return Err("PUSH: No more stack space");
        }
        self.sp -= 1;
        self.ram[self.sp] = (val >> 8) as u8;
        self.sp -= 1;
        self.ram[self.sp] = val as u8;
        Ok(())
    }

    pub fn push_reg(&mut self, reg: &str) -> EResult<()> {
        self.push(self.reg[reg])
    }

    pub fn pop(&mut self) -> EResult<u16> {
        if self.sp + 2 > self.ram.size() as u16 {
            return Err("POP: No return address on the stack");
        }
        let low = self.ram[self.sp] as u16;
        self.sp += 1;
        let high = self.ram[self.sp] as u16;
        self.sp += 1;
        Ok((high << 8) | low)
    }
    
    pub fn pop_reg(&mut self, reg: &str) -> EResult<()> {
        self.reg[reg] = self.pop()?;
        Ok(())
    }
    
    pub fn shld(&mut self, address: u16) {
        self.ram[address] = self.reg['l'];
        self.ram[address+1] = self.reg['h'];
    }
    
    pub fn lhld(&mut self, address: u16) {
        self.reg['l'] = self.ram[address];
        self.reg['h'] = self.ram[address+1];
    }
    
    pub fn sta(&mut self, address: u16) {
        self.ram[address] = self.reg['a'];
    }
    
    pub fn lda(&mut self, address: u16) {
        self.reg['a'] = self.ram[address];
    }
    
    pub fn xthl(&mut self) {
        let tempL = self.reg['l'];
        let tempH = self.reg['h'];
        self.reg['l'] = self.ram[self.sp];
        self.reg['h'] = self.ram[self.sp+1];
        self.ram[self.sp] = tempL;
        self.ram[self.sp+1] = tempH;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stax() {
        let mut emu = Emulator::new();
        
        // STAX B
        emu.ram.load_vec(vec![0x02], 0);

        emu.reg["bc"] = 0x01;
        emu.reg['a'] = 69;

        emu.execute_next().expect("Fuck");

        assert_eq!(emu.ram[0x01], 69);
    }
    
    #[test]
    fn ldax() {
        let mut emu = Emulator::new();
        
        // STAX B
        emu.ram.load_vec(vec![0x0A, 42], 0);

        emu.reg["bc"] = 0x01;
        emu.reg['a'] = 69;

        emu.execute_next().expect("Fuck");

        assert_eq!(emu.reg['a'], 42);
    }
    
    #[test]
    fn push_pop() {
        let mut emu = Emulator::new();

        emu.sp = 0x3fff;
        emu.push(0xabcd).expect("Push failed");
        assert_eq!(emu.sp, 0x3ffd);
        assert_eq!(0xabcd, emu.pop().expect("Fuck"));
        assert_eq!(emu.sp, 0x3fff);
        assert_eq!(emu.pop(), Err("POP: No return address on the stack"));

        emu.sp = 0x1;
        assert_eq!(emu.push(0x1234), Err("PUSH: No more stack space"));
    }
    
    #[test]
    fn shld() {
        let mut emu = Emulator::new();
        
        // SHLD
        emu.ram.load_vec(vec![0x22, 0x0A, 0x01], 0);

        emu.reg['h'] = 0xAE;
        emu.reg['l'] = 0x29;

        emu.execute_next().expect("Fuck");

        assert_eq!(emu.ram[0x010A], 0x29);
        assert_eq!(emu.ram[0x010B], 0xAE);
    }
    
    #[test]
    fn lhld() {
        let mut emu = Emulator::new();
        
        // LHLD
        emu.ram.load_vec(vec![0x2A, 0x0A, 0x01], 0);
        emu.ram[0x010A] = 42;
        emu.ram[0x010B] = 69;

        emu.execute_next().expect("Fuck");

        assert_eq!(emu.reg['h'], 69);
        assert_eq!(emu.reg['l'], 42);
    }
    
    #[test]
    fn sta() {
        let mut emu = Emulator::new();
        
        // STA
        emu.ram.load_vec(vec![0x32, 0x00, 0x12], 0);

        emu.reg['a'] = 69;

        emu.execute_next().expect("Fuck");

        assert_eq!(emu.ram[0x1200], 69);
    }
    
    #[test]
    fn lda() {
        let mut emu = Emulator::new();
        
        // LDA
        emu.ram.load_vec(vec![0x3A, 0x00, 0x12], 0);

        emu.ram[0x1200] = 69;
        
        emu.execute_next().expect("Fuck");

        assert_eq!(emu.reg['a'], 69);
    }
    
    #[test]
    fn xthl() {
        let mut emu = Emulator::new();
        
        // XTHL
        emu.ram.load_vec(vec![0xE3, 0x00, 0x12], 0);

        emu.reg["hl"] = 69;
        emu.sp = 0x01;
        
        emu.execute_next().expect("Fuck");

        assert_eq!(emu.reg["hl"], 0x1200);
        assert_eq!(emu.ram[0x01], 69);
    }
}

