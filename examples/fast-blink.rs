#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use nrf52833_hal::{gpio, pac};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let peripherals = pac::Peripherals::take().unwrap();
    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let _row1 = p0.p0_21.into_push_pull_output(gpio::Level::High);
    let mut row2 = p0.p0_22.into_push_pull_output(gpio::Level::High);
    let _col1 = p0.p0_28.into_push_pull_output(gpio::Level::Low);

    loop {
        row2.set_low().unwrap();
        row2.set_high().unwrap();
    }
}
