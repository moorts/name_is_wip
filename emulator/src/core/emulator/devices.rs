use super::{EResult, Emulator, InputDevice, OutputDevice};

impl<'a> Emulator<'a> {

    pub fn input(&mut self, port: u8) -> EResult<()> {
        if port >= 8 { return Err("Invalid Port") }
        match &self.input_devices[port as usize] {
            Some(device) => self.reg['a'] = device.read(),
            None => return Err("No device registered at this port")
        }
        Ok(())
    }

    pub fn output(&mut self, port: u8) -> EResult<()> {
        if port >= 8 { return Err("Invalid Port") }
        match &mut self.output_devices[port as usize] {
            Some(device) => device.write(self.reg['a']),
            None => return Err("No device registered at this port")
        }
        Ok(())
    }

    pub fn register_input_device(&mut self, device: Box<&'a dyn InputDevice>, port: usize) -> EResult<()> {
        if port >= 8 { return Err("Invalid Port") }
        self.input_devices[port] = Some(device);
        Ok(())
    }

    pub fn register_output_device(&mut self, device: Box<&'a mut dyn OutputDevice>, port: usize) -> EResult<()> {
        if port >= 8 { return Err("Invalid Port") }
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
        let logger = Logger::new();
        emu.register_input_device(Box::new(&logger), 0).expect("");

        emu.input(0).expect("");

        assert_eq!(emu.reg['a'], 42);
    }

    #[test]
    fn output() {
        let mut emu = Emulator::new();
        let mut val: u8 = 0;
        let mut logger = Logger::new();
        emu.register_output_device(Box::new(&mut logger), 0).expect("");

        emu.reg['a'] = 42;
        emu.output(0).expect("");

        assert_eq!(logger.last, 42);
    }
}
