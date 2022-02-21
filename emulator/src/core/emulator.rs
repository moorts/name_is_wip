use crate::core::ram::*;
use crate::core::register::RegisterArray;

type EResult<T> = Result<T, &'static str>;

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

    pub fn execute_next(&mut self) -> EResult<()> {
        let opcode = self.ram[self.pc];
        self.pc += 1;
        match opcode {
            0x06 => {
                // MVI B, D8
                self.mvi('b')?;
            }
            0x0e => {
                // MVI C, D8
                self.mvi('c')?;
            }
            0x16 => {
                // MVI D, D8
                self.mvi('d')?;
            }
            0x1e => {
                // MVI E, D8
                self.mvi('e')?;
            }
            0x26 => {
                // MVI H, D8
                self.mvi('h')?;
            }
            0x2e => {
                // MVI L, D8
                self.mvi('l')?;
            }
            0x36 => {
                // MVI M, D8
                self.mvi_adr()?;
            }
            0x3e => {
                // MVI A, D8
                self.mvi('a')?;
            }
            0x40..=0x7f => {
                // MOV DST, SRC
                if opcode == 0x76 {
                    // HLT
                    // TODO: Implement this
                    unimplemented!("HLT not yet supported");
                }
                self.resolve_mov(opcode)?;
            }
            0xc0 => {
                // RNZ
                self.ret_not("zero")?;
            }
            0xc1 => {
                // Unimplemented
                unimplemented!();
            }
            0xc2 => {
                // JNZ adr
                self.jmp_not("zero")?;
            }
            0xc3 => {
                // JMP adr
                self.pc = self.read_addr()?;
            }
            0xc4 => {
                // Unimplemented
                unimplemented!();
            }
            0xc5 => {
                // PUSH B
                self.push_reg("bc")?;
            }
            0xc6 => {
                // Unimplemented
                unimplemented!();
            }
            0xc7 => {
                // RST 0
                self.call(0x0)?;
            }
            0xc8 => {
                // RZ
                self.ret_if("zero")?;
            }
            0xc9 => {
                // RET
                self.ret()?;
            }
            0xca => {
                // JZ adr
                self.jmp_if("zero")?;
            }
            0xcc => {
                // CZ addr
                self.call_if("zero")?;
            }
            0xcd => {
                // CALL addr
                self.call_imm()?;
            }
            0xce => {
                // Unimplemented
                unimplemented!()
            }
            0xcf => {
                // RST 1
                self.call(0x8)?;
            }
            0xd0 => {
                // RNC
                self.ret_not("carry")?;
            }
            0xd1 => {
                // POP D
                self.reg["de"] = self.pop()?;
            }
            0xd2 => {
                // JNC adr
                self.jmp_not("carry")?;
            }
            0xd3 => {
                // OUT
                unimplemented!()
            }
            0xd4 => {
                // CNC adr
                self.call_not("carry")?;
            }
            0xd5 => {
                // PUSH D
                self.push_reg("de")?;
            }
            0xd6 => {
                // SUI D8
                unimplemented!()
            }
            0xd7 => {
                // RST 2
                self.call(0x10)?;
            }
            0xd8 => {
                // RC
                self.ret_if("carry")?;
            }
            0xd9 => {
                // no-op
                unimplemented!()
            }
            0xda => {
                // JC adr
                self.jmp_if("carry")?;
            }
            0xdb => {
                // Unimplemented
                unimplemented!()
            }
            0xdc => {
                // CC adr
                self.call_if("carry")?;
            }
            0xdd => {
                // Unimplemented
                unimplemented!()
            }
            0xde => {
                // Unimplemented
                unimplemented!()
            }
            0xdf => {
                // RST 3
                self.call(0x18)?;
            }
            0xe0 => {
                // RPO
                self.ret_not("parity")?;
            }
            0xe1 => {
                // Unimplemented
                unimplemented!()
            }
            0xe2 => {
                // JPO adr
                self.jmp_not("parity")?;
            }
            0xe3 => {
                // Unimplemented
                unimplemented!()
            }
            0xe4 => {
                // CPO adr
                self.call_not("parity")?;
            }
            0xe5 => {
                // Unimplemented
                unimplemented!()
            }
            0xe6 => {
                // Unimplemented
                unimplemented!()
            }
            0xe7 => {
                // RST 4
                self.call(0x20)?;
            }
            0xe8 => {
                // RPE
                self.ret_if("parity")?;
            }
            0xe9 => {
                // Unimplemented
                unimplemented!()
            }
            0xea => {
                // JPE adr
                self.jmp_if("parity")?;
            }
            0xeb => {
                // Unimplemented
                unimplemented!()
            }
            0xec => {
                // CPE
                self.call_if("parity")?;
            }
            0xed => {
                // Unimplemented
                unimplemented!()
            }
            0xee => {
                // Unimplemented
                unimplemented!()
            }
            0xef => {
                // RST 5
                self.call(0x28)?;
            }
            0xf0 => {
                // RP
                self.ret_not("sign")?;
            }
            0xf1 => {
                // Unimplemented
                unimplemented!()
            }
            0xf2 => {
                // JP adr
                self.jmp_not("sign")?;
            }
            0xf3 => {
                // Unimplemented
                unimplemented!()
            }
            0xf4 => {
                // CP adr
                self.call_not("sign")?;
            }
            0xf5 => {
                // Unimplemented
                unimplemented!()
            }
            0xf6 => {
                // Unimplemented
                unimplemented!()
            }
            0xf7 => {
                // RST 6
                self.call(0x30)?;
            }
            0xf8 => {
                // RM
                self.ret_if("sign")?;
            }
            0xf9 => {
                // Unimplemented
                unimplemented!()
            }
            0xfa => {
                // JM adr
                self.jmp_if("sign")?;
            }
            0xfb => {
                // Unimplemented
                unimplemented!()
            }
            0xfc => {
                // CM adr
                self.call_if("sign")?;
            }
            0xfd => {
                // Unimplemented
                unimplemented!()
            }
            0xfe => {
                // Unimplemented
                unimplemented!()
            }
            0xff => {
                // RST 7
                self.call(0x38)?;
            }
            _ => unimplemented!("Opcode not yet implemented")
        }
        Ok(())
    }

    fn mvi(&mut self, r: char) -> EResult<()> {
        self.reg[r] = self.read_byte()?;
        Ok(())
    }

    fn mvi_adr(&mut self) -> EResult<()> {
        // Move byte 2 to address in HL
        let byte = self.read_byte()?;
        let adr = self.reg["hl"];
        self.ram[adr] = byte;
        Ok(())
    }

    fn resolve_mov(&mut self, opcode: u8) -> EResult<()> {
        let registers = ['b', 'c', 'd', 'e', 'h', 'l', 'm', 'a'];
        let opcode_rel = opcode - 0x40;
        let dst_idx = opcode_rel >> 3;
        let src_idx = opcode_rel - (dst_idx << 3);
        if dst_idx == 6 {
            self.ram[self.reg["hl"]] = self.reg[registers[src_idx as usize]];
        } else {
            if src_idx == 6 {
                self.reg[registers[dst_idx as usize]] = self.ram[self.reg["hl"]];
            } else {
                self.mov(registers[dst_idx as usize], registers[src_idx as usize])?;
            }
        }
        Ok(())
    }

    fn mov(&mut self, dst: char, src: char) -> EResult<()> {
        self.reg[dst] = self.reg[src];
        Ok(())
    }

    fn jmp_not(&mut self, flag: &str) -> EResult<()> {
        if !self.reg.get_flag(flag) {
            self.pc = self.read_addr()?;
        } else {
            self.pc += 2;
        }
        Ok(())
    }

    fn jmp_if(&mut self, flag: &str) -> EResult<()> {
        if self.reg.get_flag(flag) {
            self.pc = self.read_addr()?;
        } else {
            self.pc += 2;
        }
        Ok(())
    }

    fn call_not(&mut self, flag: &str) -> EResult<()> {
        if !self.reg.get_flag(flag) {
            self.call_imm()?;
        } else {
            self.pc += 2;
        }
        Ok(())
    }

    fn call_if(&mut self, flag: &str) -> EResult<()> {
        if self.reg.get_flag(flag) {
            self.call_imm()?;
        } else {
            self.pc += 2;
        }
        Ok(())
    }

    fn call_imm(&mut self) -> EResult<()> {
        let adr = self.read_addr()?;
        self.push(self.pc)?;
        self.pc = adr;
        Ok(())
    }

    fn call(&mut self, adr: u16) -> EResult<()> {
        self.push(self.pc)?;
        self.pc = adr;
        Ok(())
    }

    fn ret_if(&mut self, flag: &str) -> EResult<()> {
        if self.reg.get_flag(flag) {
            self.ret()?;
        }
        Ok(())
    }

    fn ret_not(&mut self, flag: &str) -> EResult<()> {
        if !self.reg.get_flag(flag) {
            self.ret()?;
        }
        Ok(())
    }

    fn ret(&mut self) -> EResult<()> {
        self.pc = self.pop()?;
        Ok(())
    }

    fn push(&mut self, val: u16) -> EResult<()> {
        if self.sp < 2 {
            return Err("PUSH: No more stack space");
        }
        self.sp -= 1;
        self.ram[self.sp] = (val >> 8) as u8;
        self.sp -= 1;
        self.ram[self.sp] = val as u8;
        Ok(())
    }

    fn push_reg(&mut self, reg: &str) -> EResult<()> {
        self.push(self.reg[reg])
    }

    fn pop(&mut self) -> EResult<u16> {
        if self.sp + 2 > self.ram.size() as u16 {
            return Err("POP: No return address on the stack");
        }
        let low = self.ram[self.sp] as u16;
        self.sp += 1;
        let high = self.ram[self.sp] as u16;
        self.sp += 1;
        Ok((high << 8) | low)

    }

    fn read_byte(&mut self) -> EResult<u8> {
        if self.pc + 1 > self.ram.size() as u16 {
            return Err("READ_BYTE: Not enough bytes available");
        }
        self.pc += 1;
        Ok(self.ram[self.pc-1])
    }

    fn read_addr(&mut self) -> EResult<u16> {
        if self.pc + 2 > self.ram.size() as u16 {
            return Err("READ_ADDR: Not enough bytes available");
        }
        let low = self.ram[self.pc] as u16;
        self.pc += 1;
        let high = self.ram[self.pc] as u16;
        self.pc += 1;
        Ok((high << 8) | low)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kreator::assembler::Assembler;
    use std::{fs::*, io::{self, Read}};
    use crate::core::ram::*;

    fn load_asm_file(emulator: &mut Emulator, path: &str) -> io::Result<()> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let asmblr = Assembler::new(&buf);
        let mc = asmblr.assemble().expect("Fuck");
        emulator.ram.load_vec(mc, 0);
        Ok(())
    }

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
        Ok(())
    }

    #[test]
    fn mov_adr() {
        
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
            e.reg.set_flag(flag);
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
            e.reg.set_flag(flag);
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
        e.reg.set_flag("carry");
        e.ram.load_vec(vec![0x34, 0x12], 2);
        e.call_if("carry").expect("Fuck");
        assert_eq!(e.pc, 0x1234);
    }

    #[test]
    fn rst() {
        let mut e = Emulator::new();

        e.pc = 0x1111;
        e.sp = 0x3fff;

        e.ram.load_vec(vec![0xc7, 0xcf, 0xd7, 0xdf, 0xe7, 0xef, 0xf7, 0xff], e.pc);
        for i in 0x1111..0x1119 {
            e.pc = i as u16;
            e.execute_next().expect("Fuck");
            assert_eq!(e.pc, (i - 0x1111) * 8);
        }

    }
}
