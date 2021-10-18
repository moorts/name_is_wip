use crate::core::ram::RAM;
use crate::core::register::RegisterArray;
use crate::core::flags::Flags;


pub struct Emulator {
    pc: u16,
    sp: u16,
    acc: u8,
    ram: RAM,
    reg: RegisterArray,
    flags: Flags,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            pc: 0,
            sp: 0,
            acc: 0,
            ram: RAM::new(),
            reg: RegisterArray::new(),
            flags: Flags::new(),
        }
    }

    pub fn execute_next(&mut self) -> Result<(), &'static str> {
        let opcode = self.ram[self.pc];
        self.pc += 1;
        match opcode {
            _ => unimplemented!("Opcode not yet implemented")
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
