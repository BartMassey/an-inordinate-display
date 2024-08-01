#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::asm;
use nrf52833_pac as _;
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    // XXX should be "hello, world"
    rprintln!("hello world");

    loop {
        asm::wfi();
    }
}
