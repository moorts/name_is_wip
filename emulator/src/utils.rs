use crate::core::emulator::Emulator;
use crate::kreator::assembler::Assembler;
use std::{
    fs::*,
    io::{self, Read},
};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn load_asm_file(emulator: &mut Emulator, path: &str) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let asmblr = Assembler::new(&buf);
    let mc = asmblr.assemble().expect("Fuck");
    emulator.load_ram(mc, 0);
    Ok(())
}
