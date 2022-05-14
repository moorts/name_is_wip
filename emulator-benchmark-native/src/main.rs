use core::slice;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
use std::{fs::File, io::Read};

use emulator::{createEmulator, assemble, disassemble};
use emulator::core::io::DevNull;

fn main() {
    let mut file = File::open("8080EXM.COM").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let bufferclone = buffer.clone();

    let mut emu = createEmulator(vec![]);
    emu.load_ram(buffer, 0x100);

    emu.ram[0x00] = 0xC9;
    emu.ram[0x01] = 0x00;
    emu.ram[0x05] = 0xC9;
    emu.ram[0x06] = 0x01;
    emu.ram[0x07] = 0xC9;

    // Start at 0x100
    emu.pc = 0x100;
    emu.sp = 0xFF00;
    
    let mut totalCycles: u64 = 0;
    let mut totalInstructions: u64 = 0;
    let start = std::time::Instant::now();

    while emu.pc != 0x00 {
        let cycles = emu.execute_next().unwrap();
        totalCycles += cycles as u64; 
        totalInstructions += 1;

        if emu.pc == 0x05 {
            let syscall = emu.reg['c'];
            if syscall == 2 {
                let char = emu.reg['e'];
                print!("{}", char as char);
            } else if syscall == 9 {
                let mut address = emu.reg["de"];
                let mut currentChar: char = '\0';
                let mut fullString: String = String::new();
                while currentChar != '$' {
                    currentChar = emu.ram[address] as char;
                    if (currentChar != '$') {
                        fullString.push(currentChar);
                    }
                    address += 1;
                }
                println!("{}", fullString);
            }
        }
    }
    let end = std::time::Instant::now();
    let elapsed = end.duration_since(start);
    let elapsed_ns = elapsed.as_nanos();
    let elapsed_s = elapsed_ns / 1_000_000_000;
    println!("Elapsed time: {} ns", elapsed_ns);
    println!("Total cycles: {}", totalCycles);
    println!("Total instructions: {}", totalInstructions);
    println!("Frequency: {} MHz", (totalCycles as u128 / elapsed_s) / 1_000_000);
}
