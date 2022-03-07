use std::{cell::RefCell, rc::Rc};

use super::{EResult, Emulator, InputDevice, OutputDevice};

impl Emulator {

    pub fn input(&mut self, port: u8) -> EResult<()> {
        match &self.input_devices[port as usize] {
            Some(device) => self.reg['a'] = device.borrow().read(),
            None => return Err("No device registered at this port")
        }
        Ok(())
    }

    pub fn output(&mut self, port: u8) -> EResult<()> {
        match &self.output_devices[port as usize] {
            Some(device) => device.borrow_mut().write(self.reg['a']),
            None => return Err("No device registered at this port")
        }
        Ok(())
    }

    pub fn register_input_device(&mut self, device: Rc<RefCell<dyn InputDevice>>, port: usize) -> EResult<()> {
        self.input_devices[port] = Some(device);
        Ok(())
    }

    pub fn register_output_device(&mut self, device: Rc<RefCell<dyn OutputDevice>>, port: usize) -> EResult<()> {
        self.output_devices[port] = Some(device);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Logger {
        last: u8
    }

    impl Logger {
        fn new() -> Self {
            Logger {
                last: 0
            }
        }

        fn last(&self) -> u8 {
            self.last
        }
    }

    impl InputDevice for Logger {
        fn read(&self) -> u8 {
            42
        }
    }

    impl OutputDevice for Logger {
        fn write(&mut self, byte: u8) {
            self.last = byte;
        }
    }

    #[test]
    fn input() {
        let mut emu = Emulator::new();
        let mut val: u8 = 0;
        let logger = Rc::new(RefCell::new(Logger::new()));
        emu.register_input_device(logger.clone(), 0).expect("");

        emu.input(0).expect("");

        assert_eq!(emu.reg['a'], 42);

        assert_eq!(emu.input(1), Err("No device registered at this port"));
    }

    #[test]
    fn output() {
        let mut emu = Emulator::new();
        let mut val: u8 = 0;
        let logger = Rc::new(RefCell::new(Logger::new()));
        emu.register_output_device(logger.clone(), 0).expect("");

        emu.reg['a'] = 42;
        emu.output(0).expect("");

        assert_eq!(logger.borrow().last(), 42);
        assert_eq!(emu.output(1), Err("No device registered at this port"));
    }
}
