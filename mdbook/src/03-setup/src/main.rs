#![no_main]
#![no_std]

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use {nrf52833_pac as _, panic_rtt_target as _};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello World");
    loop {
        wfi();
    }
}
