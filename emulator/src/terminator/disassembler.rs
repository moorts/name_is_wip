use std::io;
use std::fs::File;
use std::io::prelude::Read;

use std::fmt::*;
use std::result::Result;

use num_traits::sign::Unsigned;
use num::NumCast;


struct Disassembler {
    bytes: Vec<u8>,
    pc: usize,
    out: Vec<String>,
}

impl Disassembler {

    fn load_file(path: &str) -> io::Result<Self> {
        let mut f = File::open(path)?;
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes)?;
        Ok(Disassembler { bytes, pc: 0, out: Vec::new() })
    }

    /*
     * Return byte3 + byte2
     */
    fn read_addr(&mut self) -> u16 {
        let low = self.bytes[self.pc] as u16;
        self.pc += 1;
        let high = self.bytes[self.pc] as u16;
        self.pc += 1;
        (high << 8) | low
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.bytes[self.pc];
        self.pc += 1;
        byte
    }

    fn fmt_hex<T: Unsigned + LowerHex + NumCast + Ord + Copy>(num: T) -> String {
        let mut tmp = num;
        let s: T = num::NumCast::from(16).unwrap();
        while tmp > num::NumCast::from(15).unwrap() {
            tmp = tmp / s;
        }
        if tmp < num::NumCast::from(10).unwrap() {
            return format!("{:x}H", num);
        }
        format!("0{:x}H", num)
    }

    /*
     * Decode next instruction (increments pc by 1-3)
     */
    fn decode_next(&mut self) -> Result<String, &'static str> {
        let instr = self.read_byte();
        // Jump instructions
        match instr {
            0x00 => {
                Ok(String::from("NOP"))
            },
            0x01 => {
                Ok(String::from(format!("LXI B, {}", Disassembler::fmt_hex::<u16>(self.read_addr()))))
            },
            0x02 => {
                Ok(String::from("STAX B"))
            },
            0x03 => {
                Ok(String::from("INX B"))
            },
            0x04 => {
                Ok(String::from("INR B"))
            },
            0x05 => {
                Ok(String::from("DCR B"))
            },
            0x06 => {
                Ok(String::from(format!("MVI B, {}", Disassembler::fmt_hex::<u8>(self.read_byte()))))
            },
            0x07 => {
                Ok(String::from("RLC"))
            },
            0x08 => {
                // No instruction
                Err("Invalid opcode")
            },
            0x09 => {
                Ok(String::from("DAD B"))
            },
            0x0a => {
                Ok(String::from("LDAX B"))
            },
            0x0b => {
                Ok(String::from("DCX B"))
            },
            0x0c => {
                Ok(String::from("INR C"))
            },
            0x0d => {
                Ok(String::from("DCR C"))
            },
            0x0e => {
                Ok(String::from(format!("MVI C, {}", Disassembler::fmt_hex::<u8>(self.read_byte()))))
            },
            0x0f => {
                Ok(String::from("RRC"))
            },
            0x10 => {
                // No instruction
                Err("Invalid opcode")
            },
            0x11 => {
                Ok(String::from(format!("LXI D, {}", Disassembler::fmt_hex::<u16>(self.read_addr()))))
            },
            0x12 => {
                Ok(String::from("STAX D"))
            },
            0x13 => {
                Ok(String::from("INX D"))
            },
            0x14 => {
                Ok(String::from("INR D"))
            },
            0x15 => {
                Ok(String::from("DCR D"))
            },
            0x16 => {
                Ok(String::from(format!("MVI D, {}", Disassembler::fmt_hex::<u8>(self.read_byte()))))
            },
            0x17 => {
                Ok(String::from("RAL"))
            },
            0x18 => {
                // No instruction
                Err("Invalid opcode")
            },
            0x19 => {
                Ok(String::from("DAD D"))
            },
            0x1a => {
                Ok(String::from("LDAX D"))
            },
            0x1b => {
                Ok(String::from("DCX D"))
            },
            0x1c => {
                Ok(String::from("INR E"))
            },
            0x1d => {
                Ok(String::from("DCR E"))
            },
            0x1e => {
                Ok(String::from(format!("MVI E, {}", Disassembler::fmt_hex::<u8>(self.read_byte()))))
            },
            0x1f => {
                Ok(String::from("RAR"))
            },
            0x20 => {
                Ok(String::from("RIM"))
            },
            0x21 => {
                Ok(String::from(format!("LXI H, {}", Disassembler::fmt_hex::<u16>(self.read_addr()))))
            },
            0x22 => {
                Ok(String::from(format!("SHLD {}", Disassembler::fmt_hex::<u16>(self.read_addr()))))
            },
            0x23 => {
                Ok(String::from("INX H"))
            },
            0x24 => {
                Ok(String::from("INR H"))
            },
            0x25 => {
                Ok(String::from("DCR H"))
            },
            0x26 => {
                Ok(String::from(format!("MVI H, {}", Disassembler::fmt_hex::<u8>(self.read_byte()))))
            },
            0x27 => {
                Ok(String::from("DAA"))
            },
            0x28 => {
                // No instruction
                Err("Invalid opcode")
            },
            0x29 => {
                Ok(String::from("DAD H"))
            },
            0x2a => {
                Ok(String::from(format!("LHLD {}", Disassembler::fmt_hex::<u16>(self.read_addr()))))
            },
            0x2b => {
                Ok(String::from("DCX H"))
            },
            0x2c => {
                Ok(String::from("INR L"))
            },
            0x2d => {
                Ok(String::from("DCR L"))
            },
            0x2e => {
                Ok(String::from(format!("MVI L, {}", Disassembler::fmt_hex::<u8>(self.read_byte()))))
            },
            0x2f => {
                Ok(String::from("CMA"))
            },
            0x30 => {
                Ok(String::from("SIM"))
            },
            _ => {
                Err("Invalid opcode")
            }
        }
    }

    fn disassemble(&mut self) -> Result<Vec<String>, &'static str> {
        let mut out = Vec::new();
        while self.pc < self.bytes.len() {
            out.push(self.decode_next()?);
        }
        Ok(out)
    }
}

#[test]
fn test_mov() -> io::Result<()> {
    let mut d = Disassembler { bytes: vec![0b11000011, 0xab, 0xcd], pc: 0, out: Vec::new() };
    d.decode_next();
    println!("{:?}", d.out);
    assert_eq!(1, 2);
    Ok(())
}

#[test]
fn test_fmt_hex() {
    let t1: u16 = 16;
    let t2: u16 = 15;
    let t3: u16 = 367;
    let t4: u16 = 3000;

    let t5: u8 = 16;
    let t6: u8 = 15;
    let t7: u8 = 245;

    assert_eq!(Disassembler::fmt_hex::<u16>(t1), "10H");
    assert_eq!(Disassembler::fmt_hex::<u16>(t2), "0fH");
    assert_eq!(Disassembler::fmt_hex::<u16>(t3), "16fH");
    assert_eq!(Disassembler::fmt_hex::<u16>(t4), "0bb8H");

    assert_eq!(Disassembler::fmt_hex::<u8>(t5), "10H");
    assert_eq!(Disassembler::fmt_hex::<u8>(t6), "0fH");
    assert_eq!(Disassembler::fmt_hex::<u8>(t7), "0f5H");
}
