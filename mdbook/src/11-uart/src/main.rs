#![no_main]
#![no_std]

use core::fmt::Write as _;

use cortex_m_rt::entry;
use heapless::Vec;
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

        // A buffer with 32 bytes of capacity
        let mut buffer: Vec<u8, 32> = Vec::new();

        loop {
            buffer.clear();

            loop {
                match serial.read().unwrap() {
                    b'\r' => break,
                    other => match buffer.push(other) {
                        Ok(_) => (),
                        Err(error) => write!(
                            serial,
                            "Push to buffer failed with error: {error}\r\n\0"
                        )
                        .unwrap(),
                    },
                }
            }

            buffer.iter().rev().for_each(|byte| {
                serial.write(*byte).unwrap();
            });
            serial.flush().unwrap();
        }
    } else {
        #[expect(clippy::panic, reason = "Can't avoid it anymore.")]
        {
            panic!()
        }
    }
}
