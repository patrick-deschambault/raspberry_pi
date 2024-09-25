use embedded_hal::i2c::I2c;
use linux_embedded_hal::I2cdev;
use std::sync::{Arc, Mutex};

pub const I2C_DEV_PATH: &str = "/dev/i2c-1"; // Use correct I2C bus here
pub const ADS7830_ADDR: u16 = 0x4b;

fn ads7830_commands() -> [u8; 8] {
    [0x84, 0xc4, 0x94, 0xd4, 0xa4, 0xe4, 0xb4, 0xf4]
}

fn read_ads7830(bus: &mut I2cdev, input: usize) -> Result<u8, std::io::Error> {
    let commands = ads7830_commands();

    let mut buffer = [0; 1];
    let write = [commands[input]];

    bus.write_read(ADS7830_ADDR, &write, &mut buffer).unwrap();

    Ok(buffer[0])
}

pub fn values(bus: Arc<Mutex<I2cdev>>, input: usize) -> Option<u8> {
    let mut bus = bus.lock().unwrap();

    match read_ads7830(&mut bus, input) {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
