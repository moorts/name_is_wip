use super::super::{EResult, Emulator};

const REGISTERS: [char; 8] = ['b', 'c', 'd', 'e', 'h', 'l', 'm', 'a'];

impl Emulator {
    pub fn mvi(&mut self, r: char) -> EResult<()> {
        self.reg[r] = self.read_byte()?;
        Ok(())
    }

    pub fn mvi_adr(&mut self) -> EResult<()> {
        // Move byte 2 to address in HL
        let byte = self.read_byte()?;
        let adr = self.reg["hl"];
        self.ram[adr] = byte;
        Ok(())
    }

    pub fn resolve_mov(&mut self, opcode: u8) -> EResult<()> {
        let opcode_rel = opcode - 0x40;
        let dst_idx = opcode_rel >> 3;
        let src_idx = opcode_rel - (dst_idx << 3);
        if dst_idx == 6 {
            self.ram[self.reg["hl"]] = self.reg[REGISTERS[src_idx as usize]];
        } else {
            if src_idx == 6 {
                self.reg[REGISTERS[dst_idx as usize]] = self.ram[self.reg["hl"]];
            } else {
                self.mov(REGISTERS[dst_idx as usize], REGISTERS[src_idx as usize])?;
            }
        }
        Ok(())
    }

    pub fn mov(&mut self, dst: char, src: char) -> EResult<()> {
        self.reg[dst] = self.reg[src];
        Ok(())
    }

    pub fn lxi(&mut self, dst: &str) -> EResult<()> {
        self.reg[dst] = self.read_addr()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::load_asm_file;
    use std::io;

    #[test]
    fn mvi() -> io::Result<()> {
        let mut emu = Emulator::new();
        load_asm_file(&mut emu, "./src/core/asm/mvi.s")?;

        // Check MVI reg, D8
        let regs = ['b', 'c', 'd', 'e', 'h', 'l', 'a'];
        for i in 0..7 {
            emu.execute_next().expect("Fuck");
            assert_eq!(emu.reg[regs[i]], (0x1d + i) as u8);
        }
        emu.execute_next().expect("Fuck");

        // Check MVI M, D8
        assert_eq!(emu.ram[emu.reg["hl"]], 0x24);
        Ok(())
    }

    #[test]
    fn mov() -> io::Result<()> {
        let mut emu = Emulator::new();
        load_asm_file(&mut emu, "./src/core/asm/mov.s")?;
        for _ in 0..8 {
            emu.execute_next().expect("Fuck");
        }
        for i in 0..8 {
            emu.execute_next().expect("Fuck");
            assert_eq!(emu.reg['b'], (0x1d + i) as u8);
        }

        // Test MOV M, SRC
        emu.execute_next().expect("Fuck");
        assert_eq!(emu.ram[emu.reg["hl"]], emu.reg['b']);

        // Test HLT
        emu.execute_next().expect("Fuck");
        assert_eq!(emu.running, false);
        Ok(())
    }

    #[test]
    fn lxi() -> io::Result<()> {
        let mut emu = Emulator::new();
        load_asm_file(&mut emu, "./src/core/asm/lxi.s")?;

        let regs = ["bc", "de", "hl"];
        for i in 1..4 {
            emu.execute_next().expect("Fuck");
            assert_eq!(emu.reg[regs[i - 1]], (i * 256 + i + 4) as u16);
        }
        emu.execute_next().expect("Fuck");
        assert_eq!(emu.sp, 0x0408);
        Ok(())
    }
}
