use std::time::Duration;

use raspberry_pi::lcd1602::Lcd;

fn main() {
    println!("Hello world !");
    
    let mut lcd = Lcd::new(0x27, true);

    lcd.clear();

    if lcd.init() {
        lcd.write(4, 0, "Hello");
        lcd.write(7, 1, "world!");
    }
    
    println!("Waiting for 1.0 seconds... ");
    std::thread::sleep(Duration::from_secs_f64(1.0));

    println!("Clearing");
    lcd.clear();
}
