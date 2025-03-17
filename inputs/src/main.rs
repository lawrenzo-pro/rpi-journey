use std::thread;
use std::time::Duration;
use rppal::gpio::Gpio;
fn main(){
    let btn = 17;
    let btn2 = 18;
    let red = 4;
    let blue = 3;
    let button_pin = Gpio::new().unwrap().get(btn).unwrap().into_input_pullup();
    let button_pin2 = Gpio::new().unwrap().get(btn2).unwrap().into_input_pullup();
    let mut red_pin = Gpio::new().unwrap().get(red).unwrap().into_output();
    let mut blue_pin = Gpio::new().unwrap().get(blue).unwrap().into_output();
    loop{
        if button_pin.is_low(){
            thread::sleep(Duration::from_millis(300));
            red_pin.toggle();
        }
        if button_pin2.is_low(){
            thread::sleep(Duration::from_millis(300));
            blue_pin.toggle();
        }
    }
}
