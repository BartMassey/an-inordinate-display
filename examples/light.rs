#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use nrf52833_hal::{gpio, pac};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Get a big struct "taking ownership" of peripheral registers.
    let peripherals = pac::Peripherals::take().unwrap();
    // Pull out the substruct for GPIO Port 0.
    let p0 = gpio::p0::Parts::new(peripherals.P0);
    // Grab the pins and configure them.
    let _row1: gpio::p0::P0_21<gpio::Output<gpio::PushPull>> =
        p0.p0_21.into_push_pull_output(gpio::Level::High);
    let _col1: gpio::p0::P0_28<gpio::Output<gpio::PushPull>> =
        p0.p0_28.into_push_pull_output(gpio::Level::Low);

    loop {
        asm::wfi();
    }
}
