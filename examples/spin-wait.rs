#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use nrf52833_hal::{gpio, pac};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

fn wait() {
    for _ in 0..4_000_000 {
        asm::nop();
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let peripherals = pac::Peripherals::take().unwrap();
    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let mut row1 = p0.p0_21.into_push_pull_output(gpio::Level::High);
    let _col1 = p0.p0_28.into_push_pull_output(gpio::Level::Low);

    loop {
        row1.set_low().unwrap();
        wait();
        row1.set_high().unwrap();
        wait();
    }
}
