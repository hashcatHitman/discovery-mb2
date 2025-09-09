#![no_std]
#![no_main]

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;
use gd32vf103xx_hal::delay::McycleDelay;
use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::prelude::*;
use panic_halt as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    let gpioc = dp.GPIOC.split(&mut rcu);
    let mut led = gpioc.pc13.into_push_pull_output();
    let mut delay = McycleDelay::new(&rcu.clocks);

    loop {
        delay.delay_ms(500);
        led.set_high().unwrap();
        delay.delay_ms(500);
        led.set_low().unwrap();
    }
}
