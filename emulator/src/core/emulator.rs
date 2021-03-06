use std::cell::RefCell;
use std::rc::Rc;

use crate::core::io::*;
use crate::core::ram::*;
use crate::core::register::RegisterArray;
use wasm_bindgen::prelude::wasm_bindgen;

pub type EResult<T> = Result<T, &'static str>;

static CLOCK_CYCLES: [usize; 256] = [
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4,
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4,
    4, 10, 16, 5, 5, 5, 7, 4, 4, 10, 16, 5, 5, 5, 7, 4,
    4, 10, 13, 5, 10, 10, 10, 4, 4, 10, 13, 5, 5, 5, 7, 4,
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    7, 7, 7, 7, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 7, 5,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    0, 10, 10, 10, 0, 11, 7, 11, 0, 10, 10, 10, 0, 17, 7, 11,
    0, 10, 10, 10, 0, 11, 7, 11, 0, 10, 10, 10, 0, 17, 7, 11,
    0, 10, 10, 18, 0, 11, 7, 11, 0, 5, 10, 5, 0, 17, 7, 11,
    0, 10, 10, 4, 0, 11, 7, 11, 0, 5, 10, 4, 0, 17, 7, 11
];


#[wasm_bindgen]
pub struct Emulator {
    pub pc: u16,
    pub sp: u16,
    #[wasm_bindgen(skip)]
    pub ram: Box<dyn RAM>,
    pub reg: RegisterArray,
    input_devices: [Option<Rc<RefCell<dyn InputDevice>>>; 256],
    output_devices: [Option<Rc<RefCell<dyn OutputDevice>>>; 256],
    pub running: bool,
    pub interrupts_enabled: bool,
}

#[wasm_bindgen]
impl Emulator {
    pub fn new() -> Self {
        Emulator {
            pc: 0,
            sp: 0,
            ram: Box::new(DefaultRam::new()),
            reg: RegisterArray::new(),
            input_devices: unsafe { std::mem::zeroed() },
            output_devices: unsafe { std::mem::zeroed() },
            running: true,
            interrupts_enabled: true, // INTE
        }
    }
    
    // Used for frontend to access RAM directly
    pub fn get_ram_ptr(&self) -> *const u8 {
        self.ram.get_ptr()
    }
    
    // Used for frontend to optimize RAM display
    pub fn get_last_ram_change(&self) -> u16 {
        self.ram.get_last_changed_address()
    }

    fn execute_instruction(&mut self, opcode: u8) -> EResult<usize> {
        match opcode {
            0x00 => {
                // NOP
            }
            0x01 => {
                // LXI B, D16
                self.lxi("bc")?;
            }
            0x02 => {
                // STAX B
                self.stax("bc");
            }
            0x03 => {
                // INX B
                self.inx("bc");
            }
            0x04 => {
                // INR B
                self.inr('b');
            }
            0x05 => {
                // DCR B
                self.dcr('b');
            }
            0x06 => {
                // MVI B, D8
                self.mvi('b')?;
            }
            0x07 => {
                // RLC
                self.rlc();
            }
            0x08 => {
                // NOP
            }
            0x09 => {
                // DAD B
                self.dad(self.reg["bc"]);
            }
            0x0A => {
                // LDAX B
                self.ldax("bc");
            }
            0x0B => {
                // DCX B
                self.dcx("bc");
            }
            0x0C => {
                // INR C
                self.inr('c');
            }
            0x0D => {
                // DCR C
                self.dcr('c');
            }
            0x0E => {
                // MVI C, D8
                self.mvi('c')?;
            }
            0x0F => {
                // RRC
                self.rrc();
            }
            0x10 => {
                // NOP
            }
            0x11 => {
                // LXI D, D16
                self.lxi("de")?;
            }
            0x12 => {
                // STAX D
                self.stax("de");
            }
            0x13 => {
                // INX D
                self.inx("de");
            }
            0x14 => {
                // INR D
                self.inr('d');
            }
            0x15 => {
                // DCR D
                self.dcr('d');
            }
            0x16 => {
                // MVI D, D8
                self.mvi('d')?;
            }
            0x17 => {
                // RAL
                self.ral();
            }
            0x18 => {
                // NOP
            }
            0x19 => {
                // DAD D
                self.dad(self.reg["de"]);
            }
            0x1A => {
                // LDAX D
                self.ldax("de");
            }
            0x1B => {
                // DCX D
                self.dcx("de");
            }
            0x1C => {
                // INR E
                self.inr('e');
            }
            0x1D => {
                // DCR E
                self.dcr('e');
            }
            0x1E => {
                // MVI E, D8
                self.mvi('e')?;
            }
            0x1F => {
                // RAR
                self.rar();
            }
            0x20 => {
                // NOP
            }
            0x21 => {
                // LXI H, D16
                self.lxi("hl")?;
            }
            0x22 => {
                // SHLD A16
                let address = self.read_addr()?;
                self.shld(address);
            }
            0x23 => {
                // INX H
                self.inx("hl");
            }
            0x24 => {
                // INR H
                self.inr('h');
            }
            0x25 => {
                // DCR H
                self.dcr('h');
            }
            0x26 => {
                // MVI H, D8
                self.mvi('h')?;
            }
            0x27 => {
                // DAA
                self.daa();
            }
            0x28 => {
                // NOP
            }
            0x29 => {
                // DAD H
                self.dad(self.reg["hl"]);
            }
            0x2A => {
                // LHLD A16
                let address = self.read_addr()?;
                self.lhld(address);
            }
            0x2B => {
                // DCX H
                self.dcx("hl");
            }
            0x2C => {
                // INR L
                self.inr('l');
            }
            0x2D => {
                // DCR L
                self.dcr('l');
            }
            0x2E => {
                // MVI L, D8
                self.mvi('l')?;
            }
            0x2F => {
                // CMA
                self.cma();
            }
            0x30 => {
                // NOP
            }
            0x31 => {
                // LXI SP, D16
                self.sp = self.read_addr()?;
            }
            0x32 => {
                // STA A16
                let address = self.read_addr()?;
                self.sta(address);
            }
            0x33 => {
                // INX SP
                let prev = self.sp;
                self.sp = prev.wrapping_add(1);
            }
            0x34 => {
                // INR M
                self.inr('m');
            }
            0x35 => {
                // DCR M
                self.dcr('m');
            }
            0x36 => {
                // MVI M, D8
                self.mvi_adr()?;
            }
            0x37 => {
                // STC
                self.reg.set_flag("carry", true);
            }
            0x38 => {
                // NOP
            }
            0x39 => {
                // DAD SP
                self.dad(self.sp);
            }
            0x3A => {
                // LDA A16
                let address = self.read_addr()?;
                self.lda(address);
            }
            0x3B => {
                // DCX SP
                let prev = self.sp;
                self.sp = prev.wrapping_sub(1);
            }
            0x3C => {
                // INR A
                self.inr('a');
            }
            0x3D => {
                // DCR A
                self.dcr('a');
            }
            0x3E => {
                // MVI A, D8
                self.mvi('a')?;
            }
            0x3F => {
                // CMC
                self.reg.flip_flag("carry");
            }
            0x40..=0x7f => {
                if opcode == 0x76 {
                    // HLT
                    self.running = false;
                } else {
                    // MOV DST, SRC
                    self.resolve_mov(opcode);
                }
            }
            0x80..=0x87 => {
                // ADD
                self.add(opcode, false);
            }
            0x88..=0x8F => {
                // ADC
                self.add(opcode, true);
            }
            0x90..=0x97 => {
                // SUB
                self.sub(opcode, false);
            }
            0x98..=0x9F => {
                // SBB
                self.sub(opcode, true);
            }
            0xA0..=0xA7 => {
                // ANA
                self.and(opcode);
            }
            0xA8..=0xAF => {
                // XRA
                self.xor(opcode);
            }
            0xB0..=0xB7 => {
                // ORA
                self.or(opcode);
            }
            0xB8..=0xBF => {
                // CMP
                self.cmp(opcode);
            }
            0xC0 => {
                // RNZ
                return self.ret_not("zero");
            }
            0xC1 => {
                // POP B
                self.pop_reg("bc")?;
            }
            0xC2 => {
                // JNZ adr
                self.jmp_not("zero")?;
            }
            0xC3 => {
                // JMP adr
                self.pc = self.read_addr()?;
            }
            0xC4 => {
                // CNZ adr
                return self.call_not("zero");
            }
            0xC5 => {
                // PUSH B
                self.push_reg("bc")?;
            }
            0xC6 => {
                // ADI d8
                let value = self.read_byte()?;
                self.add_value(value as u16, false);
            }
            0xC7 => {
                // RST 0
                self.call(0x0)?;
            }
            0xC8 => {
                // RZ
                return self.ret_if("zero");
            }
            0xC9 => {
                // RET
                self.ret()?;
            }
            0xCA => {
                // JZ adr
                self.jmp_if("zero")?;
            }
            0xCB => {
                // JMP adr
                self.pc = self.read_addr()?;
            }
            0xCC => {
                // CZ addr
                return self.call_if("zero");
            }
            0xCD => {
                // CALL addr
                self.call_imm()?;
            }
            0xCE => {
                // ACI d8
                let mut value = self.read_byte()? as u16;
                self.add_value(value, true);
            }
            0xCF => {
                // RST 1
                self.call(0x8)?;
            }
            0xD0 => {
                // RNC
                return self.ret_not("carry");
            }
            0xD1 => {
                // POP D
                self.pop_reg("de")?;
            }
            0xD2 => {
                // JNC adr
                self.jmp_not("carry")?;
            }
            0xD3 => {
                // OUT
                let port = self.read_byte()?;
                self.output(port)?;
            }
            0xD4 => {
                // CNC adr
                return self.call_not("carry");
            }
            0xD5 => {
                // PUSH D
                self.push_reg("de")?;
            }
            0xD6 => {
                // SUI D8
                let value = self.read_byte()?;
                self.sub_value(value, false);
            }
            0xD7 => {
                // RST 2
                self.call(0x10)?;
            }
            0xD8 => {
                // RC
                return self.ret_if("carry");
            }
            0xD9 => {
                // RET
                self.ret()?;
            }
            0xDA => {
                // JC adr
                self.jmp_if("carry")?;
            }
            0xDB => {
                // IN
                let port = self.read_byte()?;
                self.input(port)?;
            }
            0xDC => {
                // CC adr
                return self.call_if("carry");
            }
            0xDD => {
                // CALL addr
                self.call_imm()?;
            }
            0xDE => {
                // SBI d8
                let mut value = self.read_byte()?;
                self.sub_value(value, true);
            }
            0xDF => {
                // RST 3
                self.call(0x18)?;
            }
            0xE0 => {
                // RPO
                return self.ret_not("parity");
            }
            0xE1 => {
                // POP H
                self.pop_reg("hl")?;
            }
            0xE2 => {
                // JPO adr
                self.jmp_not("parity")?;
            }
            0xE3 => {
                // XTHL
                self.xthl();
            }
            0xE4 => {
                // CPO adr
                return self.call_not("parity");
            }
            0xE5 => {
                // PUSH H
                self.push_reg("hl")?;
            }
            0xE6 => {
                // ANI d8
                let value = self.read_byte()?;
                self.and_value(value);
            }
            0xE7 => {
                // RST 4
                self.call(0x20)?;
            }
            0xE8 => {
                // RPE
                return self.ret_if("parity");
            }
            0xE9 => {
                // PCHL
                self.pc = self.reg["hl"];
            }
            0xEA => {
                // JPE adr
                self.jmp_if("parity")?;
            }
            0xEB => {
                // XCHG
                let temp = self.reg["hl"];
                self.reg["hl"] = self.reg["de"];
                self.reg["de"] = temp;
            }
            0xEC => {
                // CPE
                return self.call_if("parity");
            }
            0xED => {
                // CALL addr
                self.call_imm()?;
            }
            0xEE => {
                // XRI d8
                let value = self.read_byte()?;
                self.xor_value(value);
            }
            0xEF => {
                // RST 5
                self.call(0x28)?;
            }
            0xF0 => {
                // RP
                return self.ret_not("sign");
            }
            0xF1 => {
                // POP PSW
                self.pop_reg("psw")?;
                let psw = self.reg["psw"];
                let flags = (psw & 0xFF) as u8;
                let fixedFlags = (flags & 0xD5) | 0x02;
                let fixedPSW = (psw & 0xFF00) | fixedFlags as u16;
                self.reg["psw"] = fixedPSW;
            }
            0xF2 => {
                // JP adr
                self.jmp_not("sign")?;
            }
            0xF3 => {
                // DI
                self.interrupts_enabled = false;
            }
            0xF4 => {
                // CP adr
                return self.call_not("sign");
            }
            0xF5 => {
                // PUSH PSW
                self.push_reg("psw")?;
            }
            0xF6 => {
                // ORI d8
                let value = self.read_byte()?;
                self.or_value(value);
            }
            0xF7 => {
                // RST 6
                self.call(0x30)?;
            }
            0xF8 => {
                // RM
                return self.ret_if("sign");
            }
            0xF9 => {
                // SPHL
                self.sp = self.reg["hl"];
            }
            0xFA => {
                // JM adr
                self.jmp_if("sign")?;
            }
            0xFB => {
                // EI
                self.interrupts_enabled = true;
            }
            0xFC => {
                // CM adr
                return self.call_if("sign");
            }
            0xFD => {
                // CALL addr
                self.call_imm()?;
            }
            0xFE => {
                // CPI d8
                let value = self.read_byte()?;
                self.cmp_value(value);
            }
            0xFF => {
                // RST 7
                self.call(0x38)?;
            }
        }
        Ok(CLOCK_CYCLES[opcode as usize] as usize)
    }

    #[wasm_bindgen]
    pub fn execute_next(&mut self) -> EResult<usize> {
        let opcode = self.ram[self.pc];
        self.pc += 1;
        self.execute_instruction(opcode)
    }

    fn read_byte(&mut self) -> EResult<u8> {
        if self.pc + 1 > self.ram.size() as u16 {
            return Err("READ_BYTE: Not enough bytes available");
        }
        self.pc += 1;
        Ok(self.ram[self.pc - 1])
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

    pub fn load_ram(&mut self, data: Vec<u8>, start: u16) {
        self.ram.load_vec(data, start)
    }

    pub fn interrupt(&mut self, opcode: u8) -> EResult<usize> {
        if self.interrupts_enabled {
            self.interrupts_enabled = false;
            return self.execute_instruction(opcode);
        }
        Err("Interrupts disabled")
    }
}

mod instructions;
mod devices;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::load_asm_file;
    use std::io;

    #[test]
    fn int() -> io::Result<()> {
        let mut emu = Emulator::new();
        load_asm_file(&mut emu, "./src/core/asm/int.s")?;

        emu.pc = 0x03;
        emu.sp = 0x3fff;

        // Test DI and EI
        emu.execute_next().expect("");
        assert!(!emu.interrupts_enabled);
        emu.execute_next().expect("");
        assert!(emu.interrupts_enabled);

        emu.execute_next().expect("");
        assert_eq!(emu.reg['c'], 69);

        emu.interrupt(0xC7).expect("");
        assert_eq!(emu.pc, 0);
        assert!(!emu.interrupts_enabled);

        assert_eq!(emu.interrupt(0x0), Err("Interrupts disabled"));

        emu.execute_next().expect("");
        emu.execute_next().expect("");

        assert_eq!(emu.reg['b'], 69);
        assert_eq!(emu.pc, 0x07);

        emu.execute_next().expect("");

        assert_eq!(emu.reg['h'], 69);

        // TODO: Add another test for non RST instruction interrupts
        Ok(())
    }
}
