use super::super::{Emulator, EResult};

impl Emulator {
    pub fn stax(&mut self, register: &str) -> EResult<()> {
        self.ram[self.reg[register]] = self.reg['a'];
        Ok(())
    }
    
    pub fn ldax(&mut self, register: &str) -> EResult<()> {
        self.reg['a'] = self.ram[self.reg[register]];
        Ok(())
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
    
    pub fn shld(&mut self) -> EResult<()> {
        let address: u16 = self.read_addr()?;
        self.ram[address] = self.reg['l'];
        self.ram[address+1] = self.reg['h'];
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stax() {
        let mut e = Emulator::new();
        
        // STAX B
        e.ram.load_vec(vec![0x02], 0);

        e.reg["bc"] = 0x01;
        e.reg['a'] = 69;

        e.execute_next().expect("Fuck");

        assert_eq!(e.ram[0x01], 69);
    }
    
    #[test]
    fn ldax() {
        let mut e = Emulator::new();
        
        // STAX B
        e.ram.load_vec(vec![0x0A, 42], 0);

        e.reg["bc"] = 0x01;
        e.reg['a'] = 69;

        e.execute_next().expect("Fuck");

        assert_eq!(e.reg['a'], 42);
    }
    
    #[test]
    fn push_pop() {
        let mut e = Emulator::new();

        e.sp = 0x3fff;
        e.push(0xabcd).expect("Push failed");
        assert_eq!(e.sp, 0x3ffd);
        assert_eq!(0xabcd, e.pop().expect("Fuck"));
        assert_eq!(e.sp, 0x3fff);
        assert_eq!(e.pop(), Err("POP: No return address on the stack"));

        e.sp = 0x1;
        assert_eq!(e.push(0x1234), Err("PUSH: No more stack space"));
    }
    
    #[test]
    fn shld() {
        let mut e = Emulator::new();
        
        // STAX B
        e.ram.load_vec(vec![0x22, 0x0A, 0x01], 0);

        e.reg['h'] = 0xAE;
        e.reg['l'] = 0x29;

        e.execute_next().expect("Fuck");

        assert_eq!(e.ram[0x010A], 0x29);
        assert_eq!(e.ram[0x010B], 0xAE);
    }
}

