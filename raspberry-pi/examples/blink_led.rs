use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16
// Could be a config or asked in command line by user.
const GPIO_LED: u8 = 20;

fn main() {
    match run() {
        Ok(_) => println!("All good !"),
        Err(e) => println!("Something wrong happened: {:?}", e),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    pin.set_high();
    thread::sleep(Duration::from_millis(3000));
    pin.set_low();

    Ok(())
}
