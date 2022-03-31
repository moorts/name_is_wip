
pub trait InputDevice: {
    fn read(&self, port: u8) -> u8;
}

pub trait OutputDevice {
    fn write(&mut self, port: u8, byte: u8);
}

/* Input/Output device that does nothing */
pub struct DevNull {}

impl InputDevice for DevNull {
    fn read(&self, port: u8) -> u8 {
        0
    }
}

impl OutputDevice for DevNull {
    fn write(&mut self, port: u8, byte: u8) {
        return;
    }
}
