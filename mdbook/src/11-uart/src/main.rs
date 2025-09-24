#![no_main]
#![no_std]

use core::fmt::Write as _;

use cortex_m_rt::entry;
use microbit::hal::uarte;
use microbit::hal::uarte::{Baudrate, Parity};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;
use serial_setup::UartePort;

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

        loop {
            let byte: char = serial.read().unwrap().into();
            write!(serial, "{byte}").unwrap();
            serial.flush().unwrap();
        }
    } else {
        #[expect(clippy::panic, reason = "Can't avoid it anymore.")]
        {
            panic!()
        }
    }
}
