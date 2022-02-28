use super::super::{Emulator, EResult};

impl Emulator {
    pub fn stax(&mut self, register: &str) -> EResult<()> {
        self.ram[self.reg[register]] = self.reg['a'];
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
}

