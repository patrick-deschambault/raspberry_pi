use std::{process::exit, sync::{Arc, Mutex}, thread, time::Duration};

use linux_embedded_hal::I2cdev;
use raspberry_pi::adc7830::{values, I2C_DEV_PATH};
use rppal::pwm::Polarity;
use rppal::pwm::{Channel, Pwm};

fn main() {
    
    let bus = Arc::new(Mutex::new(I2cdev::new(I2C_DEV_PATH).unwrap()));
    //let pwm = Pwm::with_period(Channel::Pwm0, Duration::from_millis(100), Duration::from_millis(1), Polarity::Normal, true).unwrap();
    //let pwm = Arc::new(pwm);
    
    let bus_clone = Arc::clone(&bus);

    ctrlc::set_handler({
        //let pwm = pwm.clone();
        move || {
        println!("Exiting gracefully...");
        //pwm.disable().unwrap();
        exit(0);
    }}).expect("Error setting Ctrl-C handler");

    thread::spawn(move || {
        for value in values(bus_clone, 0) {
            //pwm.set_duty_cycle(value as f64).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Infinite loop to keep the program running
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}