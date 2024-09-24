use embedded_hal::i2c::I2c;
use linux_embedded_hal::I2cdev;
use std::{thread::sleep, time::Duration};

pub struct Lcd {
    addr: u8,
    bus: I2cdev,
    blen: bool,
}

impl Lcd {
    pub fn new(addr: u8, blen: bool) -> Self {
        let bus = I2cdev::new("/dev/i2c-1").unwrap();
        Lcd { addr, bus, blen }
    }

    fn write_word(&mut self, data: u8) {
        let mut temp = data;
        if self.blen {
            temp |= 0x08;
        } else {
            temp &= 0xF7;
        }

        self.bus.write(self.addr, &[temp]).unwrap();
    }

    fn send_command(&mut self, comm: u8) {
        // Send bit7-4 first
        let mut buf = comm & 0xF0;
        buf |= 0x04; // RS = 0, RW = 0, EN = 1
        self.write_word(buf);
        sleep(Duration::from_millis(2));
        buf &= 0xFB; // EN = 0
        self.write_word(buf);

        // Send bit3-0
        buf = (comm & 0x0F) << 4;
        buf |= 0x04; // RS = 0, RW = 0, EN = 1
        self.write_word(buf);
        sleep(Duration::from_millis(2));
        buf &= 0xFB; // EN = 0
        self.write_word(buf);
    }

    fn send_data(&mut self, data: u8) {
        // Send bit7-4 first
        let mut buf = data & 0xF0;
        buf |= 0x05; // RS = 1, RW = 0, EN = 1
        self.write_word(buf);
        sleep(Duration::from_millis(2));
        buf &= 0xFB; // EN = 0
        self.write_word(buf);

        // Send bit3-0
        buf = (data & 0x0F) << 4;
        buf |= 0x05; // RS = 1, RW = 0, EN = 1
        self.write_word(buf);
        sleep(Duration::from_millis(2));
        buf &= 0xFB; // EN = 0
        self.write_word(buf);
    }

    pub fn init(&mut self) -> bool {
        if self.send_init_commands() {
            self.bus.write(self.addr, &[0x08]).unwrap();
            true
        } else {
            false
        }
    }

    fn send_init_commands(&mut self) -> bool {
        let commands = [0x33, 0x32, 0x28, 0x0C, 0x01];
        for &cmd in &commands {
            self.send_command(cmd);
            sleep(Duration::from_millis(5));
        }
        true
    }

    pub fn clear(&mut self) {
        self.send_command(0x01); // Clear Screen
    }

    fn open_light(&mut self) {
        self.bus.write(0x27 as u8, &[0x08]).unwrap();
    }

    pub fn write(&mut self, x: i32, y: i32, text: &str) {
        let x = x.clamp(0, 15);
        let y = y.clamp(0, 1);
        let addr = 0x80 + 0x40 * y as u8 + x as u8;
        self.send_command(addr);

        for chr in text.chars() {
            self.send_data(chr as u8);
        }
    }
}
