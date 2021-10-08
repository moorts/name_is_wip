use std::fs::File;
use std::io;
use std::io::prelude::Read;

use std::fmt::*;
use std::result::Result;

use num::NumCast;
use num_traits::sign::Unsigned;

struct Disassembler {
    bytes: Vec<u8>,
    pc: usize,
}

impl Iterator for Disassembler {
    type Item = Result<String, &'static str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pc < self.bytes.len() {
            return Some(self.decode_next());
        }
        None
    }
}

impl Disassembler {
    fn load_file(path: &str) -> io::Result<Self> {
        let mut f = File::open(path)?;
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes)?;
        Ok(Disassembler { bytes, pc: 0 })
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
            0x00 => Ok(String::from("NOP")),
            0x01 => Ok(String::from(format!(
                "LXI B,{}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            ))),
            0x02 => Ok(String::from("STAX B")),
            0x03 => Ok(String::from("INX B")),
            0x04 => Ok(String::from("INR B")),
            0x05 => Ok(String::from("DCR B")),
            0x06 => Ok(String::from(format!(
                "MVI B,{}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            ))),
            0x07 => Ok(String::from("RLC")),
            0x08 => {
                // No instruction
                Err("Invalid opcode")
            }
            0x09 => Ok(String::from("DAD B")),
            0x0a => Ok(String::from("LDAX B")),
            0x0b => Ok(String::from("DCX B")),
            0x0c => Ok(String::from("INR C")),
            0x0d => Ok(String::from("DCR C")),
            0x0e => Ok(String::from(format!(
                "MVI C,{}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            ))),
            0x0f => Ok(String::from("RRC")),
            0x10 => {
                // No instruction
                Err("Invalid opcode")
            }
            0x11 => Ok(String::from(format!(
                "LXI D,{}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            ))),
            0x12 => Ok(String::from("STAX D")),
            0x13 => Ok(String::from("INX D")),
            0x14 => Ok(String::from("INR D")),
            0x15 => Ok(String::from("DCR D")),
            0x16 => Ok(String::from(format!(
                "MVI D,{}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            ))),
            0x17 => Ok(String::from("RAL")),
            0x18 => {
                // No instruction
                Err("Invalid opcode")
            }
            0x19 => Ok(String::from("DAD D")),
            0x1a => Ok(String::from("LDAX D")),
            0x1b => Ok(String::from("DCX D")),
            0x1c => Ok(String::from("INR E")),
            0x1d => Ok(String::from("DCR E")),
            0x1e => Ok(String::from(format!(
                "MVI E,{}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            ))),
            0x1f => Ok(String::from("RAR")),
            0x20 => Ok(String::from("RIM")),
            0x21 => Ok(String::from(format!(
                "LXI H,{}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            ))),
            0x22 => Ok(String::from(format!(
                "SHLD {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            ))),
            0x23 => Ok(String::from("INX H")),
            0x24 => Ok(String::from("INR H")),
            0x25 => Ok(String::from("DCR H")),
            0x26 => Ok(String::from(format!(
                "MVI H,{}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            ))),
            0x27 => Ok(String::from("DAA")),
            0x28 => {
                // No instruction
                Err("Invalid opcode")
            }
            0x29 => Ok(String::from("DAD H")),
            0x2a => Ok(String::from(format!(
                "LHLD {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            ))),
            0x2b => Ok(String::from("DCX H")),
            0x2c => Ok(String::from("INR L")),
            0x2d => Ok(String::from("DCR L")),
            0x2e => Ok(String::from(format!(
                "MVI L,{}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            ))),
            0x2f => Ok(String::from("CMA")),
            0x30 => Ok(String::from("SIM")),
            0x31 => Ok(String::from(format!(
                "LXI SP,{}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            ))),
            0x32 => Ok(String::from(format!(
                "STA {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            ))),
            0x33 => Ok(String::from("INX SP")),
            0x34 => Ok(String::from("INR M")),
            0x35 => Ok(String::from("DCR M")),
            0x36 => Ok(String::from(format!(
                "MVI M,{}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            ))),
            0x37 => Ok(String::from("STC")),
            0x38 => {
                // No instruction
                Err("Invalid opcode")
            }
            0x39 => Ok(String::from("DAD SP")),
            0x3a => Ok(String::from(format!(
                "LDA {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            ))),
            0x3b => Ok(String::from("DCX SP")),
            0x3c => Ok(String::from("INR A")),
            0x3d => Ok(String::from("DCR A")),
            0x3e => Ok(String::from(format!(
                "MVI A,{}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            ))),
            0x3f => Ok(String::from("CMC")),
            0x40 => Ok(String::from("MOV B,B")),
            0x41 => Ok(String::from("MOV B,C")),
            0x42 => Ok(String::from("MOV B,D")),
            0x43 => Ok(String::from("MOV B,E")),
            0x44 => Ok(String::from("MOV B,H")),
            0x45 => Ok(String::from("MOV B,L")),
            0x46 => Ok(String::from("MOV B,M")),
            0x47 => Ok(String::from("MOV B,A")),
            0x48 => Ok(String::from("MOV C,B")),
            0x49 => Ok(String::from("MOV C,C")),
            0x4a => Ok(String::from("MOV C,D")),
            0x4b => Ok(String::from("MOV C,E")),
            0x4c => Ok(String::from("MOV C,H")),
            0x4d => Ok(String::from("MOV C,L")),
            0x4e => Ok(String::from("MOV C,M")),
            0x4f => Ok(String::from("MOV C,A")),
            0x50 => Ok(String::from("MOV D,B")),
            0x51 => Ok(String::from("MOV D,C")),
            0x52 => Ok(String::from("MOV D,D")),
            0x53 => Ok(String::from("MOV D,E")),
            0x54 => Ok(String::from("MOV D,H")),
            0x55 => Ok(String::from("MOV D,L")),
            0x56 => Ok(String::from("MOV D,M")),
            0x57 => Ok(String::from("MOV D,A")),
            0x58 => Ok(String::from("MOV E,B")),
            0x59 => Ok(String::from("MOV E,C")),
            0x5a => Ok(String::from("MOV E,D")),
            0x5b => Ok(String::from("MOV E,E")),
            0x5c => Ok(String::from("MOV E,H")),
            0x5d => Ok(String::from("MOV E,L")),
            0x5e => Ok(String::from("MOV E,M")),
            0x5f => Ok(String::from("MOV E,A")),
            0x60 => Ok(String::from("MOV H,B")),
            0x61 => Ok(String::from("MOV H,C")),
            0x62 => Ok(String::from("MOV H,D")),
            0x63 => Ok(String::from("MOV H,E")),
            0x64 => Ok(String::from("MOV H,H")),
            0x65 => Ok(String::from("MOV H,L")),
            0x66 => Ok(String::from("MOV H,M")),
            0x67 => Ok(String::from("MOV H,A")),
            0x68 => Ok(String::from("MOV L,B")),
            0x69 => Ok(String::from("MOV L,C")),
            0x6a => Ok(String::from("MOV L,D")),
            0x6b => Ok(String::from("MOV L,E")),
            0x6c => Ok(String::from("MOV L,H")),
            0x6d => Ok(String::from("MOV L,L")),
            0x6e => Ok(String::from("MOV L,M")),
            0x6f => Ok(String::from("MOV L,A")),
            0x70 => Ok(String::from("MOV M,B")),
            0x71 => Ok(String::from("MOV M,C")),
            0x72 => Ok(String::from("MOV M,D")),
            0x73 => Ok(String::from("MOV M,E")),
            0x74 => Ok(String::from("MOV M,H")),
            0x75 => Ok(String::from("MOV M,L")),
            0x76 => Ok(String::from("HLT")),
            0x77 => Ok(String::from("MOV M,A")),
            0x78 => Ok(String::from("MOV A,B")),
            0x79 => Ok(String::from("MOV A,C")),
            0x7a => Ok(String::from("MOV A,D")),
            0x7b => Ok(String::from("MOV A,E")),
            0x7c => Ok(String::from("MOV A,H")),
            0x7d => Ok(String::from("MOV A,L")),
            0x7e => Ok(String::from("MOV A,M")),
            0x7f => Ok(String::from("MOV A,A")),
            _ => Err("Invalid opcode"),
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::BufRead;

    const OPCODE_TEST_DATA: &str = "./test_data/test_input";

    #[test]
    fn test_opcodes() -> io::Result<()> {
        let f = File::open(OPCODE_TEST_DATA)?;
        let lines = io::BufReader::new(f).lines();

        let mut d = Disassembler {
            bytes: Vec::new(),
            pc: 0,
        };
        let mut outputs = Vec::new();
        for line in lines {
            if let Ok(data) = line {
                let mut split = data.split(":");
                let bytes = split
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|x| x.to_string().parse::<u8>().unwrap())
                    .into_iter();
                d.bytes.extend(bytes);
                outputs.push(String::from(split.next().unwrap()));
            }
        }
        for output in outputs {
            let disassembly = match d.next().unwrap() {
                Ok(x) => x,
                Err(_) => String::from("-"),
            };
            assert_eq!(disassembly, output);
        }
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
}
