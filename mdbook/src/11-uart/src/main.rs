#![no_main]
#![no_std]

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use microbit::hal::uarte;
use microbit::hal::uarte::{Baudrate, Parity};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;
use serial_setup::UartePort;

const PAYLOAD: &str = "The quick brown fox jumps over the lazy dog.\n\0";

#[entry]
fn main() -> ! {
    rtt_init_print!();

    if let Some(board) = microbit::Board::take() {
        let mut serial = {
            let serial = uarte::Uarte::new(
                board.UARTE0,
                board.uart.into(),
                Parity::EXCLUDED,
                Baudrate::BAUD115200,
            );
            UartePort::new(serial)
        };

        PAYLOAD.bytes().for_each(|byte| {
            serial.write(byte).unwrap();
        });
        serial.flush().unwrap();

        loop {
            wfi();
        }
    } else {
        #[expect(clippy::panic, reason = "Can't avoid it anymore.")]
        {
            panic!()
        }
    }
}
