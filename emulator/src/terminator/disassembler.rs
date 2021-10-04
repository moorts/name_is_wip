use std::io;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::Read;
use std::collections::HashMap;

enum Instruction {
    MOVRR(char, char),
    MOVMR(u16, char),
    MOVRM(char, u16),
    HLT,
    MVIR(char),
    MVIM(u8),
    INR(char),
    DCR(char),
    INR,
    DCR,
    // -------
    ADD(char),
    ADC(char),
    SUB(char),
    SBB(char),
    ANA(char),
    XRA(char),
    ORA(char),
    CMP(char),
    ADD,
    ADC,
    SUB,
    SBB,
    ANA,
    XRA,
    ORA,
    CMP,
    ADI(u8),
    ACI(u8),
    SUI(u8),
    SBI(u8),
    ANI(u8),
    XRI(u8),
    ORI(u8),
    CPI(u8),
    RLC,
    RRC,
    RAL,
    RAR,
    JMP(u16),
    JC(u16),
    JNC(u16),
    JZ(u16),
    JNZ(u16),
    JP(u16),
    JM(u16),
    JPE(u16),
    JPO(u16),
}

enum Param {
    Addr(u16),
    Imm(u8),
    Reg(char),
}

struct Instr {
    opcode: String,
    arg1: Option<Param>,
    arg2: Option<Param>,
}

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
        self.pc += 1;
        let low = self.bytes[self.pc] as u16;
        self.pc += 1;
        let high = self.bytes[self.pc] as u16;
        (high << 8) | low
    }

    /*
     * Decode next instruction (increments pc by 1-3)
     */
    fn decode_next(&mut self) {
        let instr = self.bytes[self.pc];
        // Jump instructions
        match instr {
            0b11000011 => {
                let addr = self.read_addr();
                self.out.push(String::from(format!("JMP {:x}", addr)));
            },
            0b11011010 => {
                let addr = self.read_addr();
                self.out.push(String::from(format!("JC {:x}", addr)));
            },
            0b11010010 => {
                let addr = self.read_addr();
                self.out.push(String::from(format!("JNC {:x}", addr)));
            },
            0b11001010 => {
                let addr = self.read_addr();
                self.out.push(String::from(format!("JZ {:x}", addr)));
            },
            0b11000010 => {
                let addr = self.read_addr();
                self.out.push(String::from(format!("JNZ {:x}", addr)));
            },
            0b11110010 => {
                let addr = self.read_addr();
                self.out.push(String::from(format!("JP {:x}", addr)));
            },
            0b11111010 => {
                let addr = self.read_addr();
                self.out.push(String::from(format!("JM {:x}", addr)));
            },
            0b11101010 => {
                let addr = self.read_addr();
                self.out.push(String::from(format!("JPE {:x}", addr)));
            },
            0b11100010 => {
                let addr = self.read_addr();
                self.out.push(String::from(format!("JPO {:x}", addr)));
            },
            _ => {

            }
        }
        match instr & 0b11000000 {
            0b01000000 => {
                if instr == 0b01110110 {
                    self.out.push(String::from("HLT"));
                }

            },
            _ => {

            },
        }

    }

    fn disassemble(&mut self) {
        while self.pc < self.bytes.len() {
            self.decode_next();
        }
    }
}

#[test]
fn test_mov() -> io::Result<()> {
    let lol = Instruction::Double(1, 2);
    let yikes = Instruction::Hello(5);
    let mut d = Disassembler { bytes: vec![0b11000011, 0xab, 0xcd], pc: 0, out: Vec::new() };
    d.decode_next();
    println!("{:?}", d.out);
    assert_eq!(1, 2);
    Ok(())
}
