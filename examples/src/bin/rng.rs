//! Demonstrates the use of the hardware Random Number Generator (RNG)

//% CHIPS: esp32 esp32c2 esp32c3 esp32c6 esp32h2 esp32s2 esp32s3
//% FEATURES: embedded-hal-02

#![no_std]
#![no_main]

use embedded_hal_02::blocking::rng::Read;
use esp_backtrace as _;
use esp_hal::{peripherals::Peripherals, prelude::*, rng::Rng};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut rng = Rng::new(peripherals.RNG);

    // Generate a random word (u32):
    println!("Random u32:   {}", rng.random());

    // Fill a buffer with random bytes:
    let mut buf = [0u8; 16];
    rng.read(&mut buf).unwrap();
    println!("Random bytes: {:?}", buf);

    loop {}
}
