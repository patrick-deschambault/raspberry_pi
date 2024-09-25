use std::{
    f64::consts::LOG10_2,
    process::exit,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use linux_embedded_hal::I2cdev;
use raspberry_pi::ads7830::{values, I2C_DEV_PATH};
use rppal::pwm::Polarity;
use rppal::pwm::{Channel, Pwm};

const STEPS: f64 = 255.0;

fn main() {
    let bus = Arc::new(Mutex::new(I2cdev::new(I2C_DEV_PATH).unwrap()));
    let pwm = Pwm::with_period(
        Channel::Pwm0,
        Duration::from_millis(1),
        Duration::from_millis(1),
        Polarity::Normal,
        true,
    )
    .unwrap();
    let pwm = Arc::new(pwm);

    ctrlc::set_handler({
        let pwm = pwm.clone();
        move || {
            println!("Exiting gracefully...");
            pwm.disable().unwrap();
            exit(0);
        }
    })
    .expect("Error setting Ctrl-C handler");

    thread::spawn(move || loop {
        let value = values(bus.clone(), 0);

        if let Some(v) = value {
            let fade_factor = fade_factor();
            let duty = (2f64.powf(v as f64 / fade_factor) - 1.0) / STEPS;
            pwm.set_duty_cycle(duty).unwrap();
            println!("{:?}, {:?}", v, duty);
        }

        thread::sleep(Duration::from_millis(50));
    });

    // Infinite loop to keep the program running
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

fn fade_factor() -> f64 {
    (STEPS * LOG10_2) / f64::log10(STEPS)
}
