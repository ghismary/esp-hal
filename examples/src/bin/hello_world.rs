//! This shows how to write text to UART0.
//!
//! You can see the output with `espflash` if you provide the `--monitor`
//! option. Depending on the chip, you will need to ensure that you are
//! connected to the UART USB port, and not the USB-SERIAL-JTAG port.
//! If you want to test printing over USB-SERIAL-JTAG, try the usb_serial_jtag
//! example instead.

//% CHIPS: esp32 esp32c2 esp32c3 esp32c6 esp32h2 esp32s2 esp32s3
//% FEATURES: embedded-hal-02

#![no_std]
#![no_main]

use core::fmt::Write;

use embedded_hal_02::timer::CountDown;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    uart::Uart,
};
use nb::block;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut timer0 = timg0.timer0;
    timer0.start(1u64.secs());

    let mut uart0 = Uart::new(peripherals.UART0, &clocks);

    loop {
        writeln!(uart0, "Hello world!").unwrap();
        block!(timer0.wait()).unwrap();
    }
}
