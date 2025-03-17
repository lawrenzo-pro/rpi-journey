use std::error::Error;
use std::thread;
use std::time::Duration;
use rppal::{gpio::Gpio,system::DeviceInfo};
fn main() -> Result<(),Box<dyn Error>>{
    let led1_pin = 3;
    let led2_pin = 4;

    println!("Running on a: {} ", DeviceInfo::new()?.model());
    let mut led1 = Gpio::new()?.get(led1_pin)?.into_output();
    let mut led2 = Gpio::new()?.get(led2_pin)?.into_output();
    thread::spawn(move||{
        loop{
            led1.set_high();
            thread::sleep(Duration::from_millis(197));
            led1.set_low();
            thread::sleep(Duration::from_millis(197));
        }
    });
    loop{
        led2.set_high();
        thread::sleep(Duration::from_millis(200));
        led2.set_low();
        thread::sleep(Duration::from_millis(200));
    }
    Ok(())
}
