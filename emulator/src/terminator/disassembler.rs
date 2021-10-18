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
            0x01 => Ok(format!(
                "LXI B,{}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0x02 => Ok(String::from("STAX B")),
            0x03 => Ok(String::from("INX B")),
            0x04 => Ok(String::from("INR B")),
            0x05 => Ok(String::from("DCR B")),
            0x06 => Ok(format!(
                "MVI B,{}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
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
            0x0e => Ok(format!(
                "MVI C,{}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
            0x0f => Ok(String::from("RRC")),
            0x10 => {
                // No instruction
                Err("Invalid opcode")
            }
            0x11 => Ok(format!(
                "LXI D,{}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0x12 => Ok(String::from("STAX D")),
            0x13 => Ok(String::from("INX D")),
            0x14 => Ok(String::from("INR D")),
            0x15 => Ok(String::from("DCR D")),
            0x16 => Ok(format!(
                "MVI D,{}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
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
            0x1e => Ok(format!(
                "MVI E,{}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
            0x1f => Ok(String::from("RAR")),
            0x20 => Ok(String::from("RIM")),
            0x21 => Ok(format!(
                "LXI H,{}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0x22 => Ok(format!(
                "SHLD {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0x23 => Ok(String::from("INX H")),
            0x24 => Ok(String::from("INR H")),
            0x25 => Ok(String::from("DCR H")),
            0x26 => Ok(format!(
                "MVI H,{}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
            0x27 => Ok(String::from("DAA")),
            0x28 => {
                // No instruction
                Err("Invalid opcode")
            }
            0x29 => Ok(String::from("DAD H")),
            0x2a => Ok(format!(
                "LHLD {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0x2b => Ok(String::from("DCX H")),
            0x2c => Ok(String::from("INR L")),
            0x2d => Ok(String::from("DCR L")),
            0x2e => Ok(format!(
                "MVI L,{}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
            0x2f => Ok(String::from("CMA")),
            0x30 => Ok(String::from("SIM")),
            0x31 => Ok(format!(
                "LXI SP,{}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0x32 => Ok(format!(
                "STA {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0x33 => Ok(String::from("INX SP")),
            0x34 => Ok(String::from("INR M")),
            0x35 => Ok(String::from("DCR M")),
            0x36 => Ok(format!(
                "MVI M,{}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
            0x37 => Ok(String::from("STC")),
            0x38 => {
                // No instruction
                Err("Invalid opcode")
            }
            0x39 => Ok(String::from("DAD SP")),
            0x3a => Ok(format!(
                "LDA {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0x3b => Ok(String::from("DCX SP")),
            0x3c => Ok(String::from("INR A")),
            0x3d => Ok(String::from("DCR A")),
            0x3e => Ok(format!(
                "MVI A,{}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
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
            0x80 => Ok(String::from("ADD B")),
            0x81 => Ok(String::from("ADD C")),
            0x82 => Ok(String::from("ADD D")),
            0x83 => Ok(String::from("ADD E")),
            0x84 => Ok(String::from("ADD H")),
            0x85 => Ok(String::from("ADD L")),
            0x86 => Ok(String::from("ADD M")),
            0x87 => Ok(String::from("ADD A")),
            0x88 => Ok(String::from("ADC B")),
            0x89 => Ok(String::from("ADC C")),
            0x8a => Ok(String::from("ADC D")),
            0x8b => Ok(String::from("ADC E")),
            0x8c => Ok(String::from("ADC H")),
            0x8d => Ok(String::from("ADC L")),
            0x8e => Ok(String::from("ADC M")),
            0x8f => Ok(String::from("ADC A")),
            0x90 => Ok(String::from("SUB B")),
            0x91 => Ok(String::from("SUB C")),
            0x92 => Ok(String::from("SUB D")),
            0x93 => Ok(String::from("SUB E")),
            0x94 => Ok(String::from("SUB H")),
            0x95 => Ok(String::from("SUB L")),
            0x96 => Ok(String::from("SUB M")),
            0x97 => Ok(String::from("SUB A")),
            0x98 => Ok(String::from("SBB B")),
            0x99 => Ok(String::from("SBB C")),
            0x9a => Ok(String::from("SBB D")),
            0x9b => Ok(String::from("SBB E")),
            0x9c => Ok(String::from("SBB H")),
            0x9d => Ok(String::from("SBB L")),
            0x9e => Ok(String::from("SBB M")),
            0x9f => Ok(String::from("SBB A")),
            0xa0 => Ok(String::from("ANA B")),
            0xa1 => Ok(String::from("ANA C")),
            0xa2 => Ok(String::from("ANA D")),
            0xa3 => Ok(String::from("ANA E")),
            0xa4 => Ok(String::from("ANA H")),
            0xa5 => Ok(String::from("ANA L")),
            0xa6 => Ok(String::from("ANA M")),
            0xa7 => Ok(String::from("ANA A")),
            0xa8 => Ok(String::from("XRA B")),
            0xa9 => Ok(String::from("XRA C")),
            0xaa => Ok(String::from("XRA D")),
            0xab => Ok(String::from("XRA E")),
            0xac => Ok(String::from("XRA H")),
            0xad => Ok(String::from("XRA L")),
            0xae => Ok(String::from("XRA M")),
            0xaf => Ok(String::from("XRA A")),
            0xb0 => Ok(String::from("ORA B")),
            0xb1 => Ok(String::from("ORA C")),
            0xb2 => Ok(String::from("ORA D")),
            0xb3 => Ok(String::from("ORA E")),
            0xb4 => Ok(String::from("ORA H")),
            0xb5 => Ok(String::from("ORA L")),
            0xb6 => Ok(String::from("ORA M")),
            0xb7 => Ok(String::from("ORA A")),
            0xb8 => Ok(String::from("CMP B")),
            0xb9 => Ok(String::from("CMP C")),
            0xba => Ok(String::from("CMP D")),
            0xbb => Ok(String::from("CMP E")),
            0xbc => Ok(String::from("CMP H")),
            0xbd => Ok(String::from("CMP L")),
            0xbe => Ok(String::from("CMP M")),
            0xbf => Ok(String::from("CMP A")),
            0xc0 => Ok(String::from("RNZ")),
            0xc1 => Ok(String::from("POP B")),
            0xc2 => Ok(format!(
                "JNZ {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xc3 => Ok(format!(
                "JMP {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xc4 => Ok(format!(
                "CNZ {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xc5 => Ok(String::from("PUSH B")),
            0xc6 => Ok(format!(
                "ADI {}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
            0xc7 => Ok(String::from("RST 0")),
            0xc8 => Ok(String::from("RZ")),
            0xc9 => Ok(String::from("RET")),
            0xca => Ok(format!(
                "JZ {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xcb => {
                // No instruction
                Err("Invalid opcode")
            }
            0xcc => Ok(format!(
                "CZ {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xcd => Ok(format!(
                "CALL {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xce => Ok(format!(
                "ACI {}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
            0xcf => Ok(String::from("RST 1")),
            0xd0 => Ok(String::from("RNC")),
            0xd1 => Ok(String::from("POP D")),
            0xd2 => Ok(format!(
                "JNC {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xd3 => Ok(format!(
                "OUT {}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
            0xd4 => Ok(format!(
                "CNC {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xd5 => Ok(String::from("PUSH D")),
            0xd6 => Ok(format!(
                "SUI {}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
            0xd7 => Ok(String::from("RST 2")),
            0xd8 => Ok(String::from("RC")),
            0xd9 => {
                // No instruction
                Err("Invalid opcode")
            }
            0xda => Ok(format!(
                "JC {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xdb => Ok(format!(
                "IN {}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
            0xdc => Ok(format!(
                "CC {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xdd => {
                // No instruction
                Err("Invalid opcode")
            }
            0xde => Ok(format!(
                "SBI {}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
            0xdf => Ok(String::from("RST 3")),
            0xe0 => Ok(String::from("RPO")),
            0xe1 => Ok(String::from("POP H")),
            0xe2 => Ok(format!(
                "JPO {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xe3 => Ok(String::from("XTHL")),
            0xe4 => Ok(format!(
                "CPO {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xe5 => Ok(String::from("PUSH H")),
            0xe6 => Ok(format!(
                "ANI {}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
            0xe7 => Ok(String::from("RST 4")),
            0xe8 => Ok(String::from("RPE")),
            0xe9 => Ok(String::from("PCHL")),
            0xea => Ok(format!(
                "JPE {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xeb => Ok(String::from("XCHG")),
            0xec => Ok(format!(
                "CPE {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xed => {
                // No instruction
                Err("Invalid opcode")
            }
            0xee => Ok(format!(
                "XRI {}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
            0xef => Ok(String::from("RST 5")),
            0xf0 => Ok(String::from("RP")),
            0xf1 => Ok(String::from("POP PSW")),
            0xf2 => Ok(format!(
                "JP {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xf3 => Ok(String::from("DI")),
            0xf4 => Ok(format!(
                "CP {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xf5 => Ok(String::from("PUSH PSW")),
            0xf6 => Ok(format!(
                "ORI {}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
            0xf7 => Ok(String::from("RST 6")),
            0xf8 => Ok(String::from("RM")),
            0xf9 => Ok(String::from("SPHL")),
            0xfa => Ok(format!(
                "JM {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xfb => Ok(String::from("EI")),
            0xfc => Ok(format!(
                "CM {}",
                Disassembler::fmt_hex::<u16>(self.read_addr())
            )),
            0xfd => {
                // No instruction
                Err("Invalid opcode")
            }
            0xfe => Ok(format!(
                "CPI {}",
                Disassembler::fmt_hex::<u8>(self.read_byte())
            )),
            0xff => Ok(String::from("RST 7")),
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
