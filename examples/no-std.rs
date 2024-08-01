#![no_std]
#![no_main]

// Sets up our `main` to be linked as the entry point.
use cortex_m_rt::entry;
// Sets up linking of interrupt vectors etc.
use nrf52833_pac as _;

// Rust calls this when a panic happens.
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    // No OS kernel to return to, so just sit and spin.
    #[allow(clippy::needless_loop)]
    loop {}
}

#[entry]
fn main() -> ! {
    panic!()
}
