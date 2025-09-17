#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
use microbit::Board;
use microbit::display::blocking::Display;
use microbit::hal::timer::Timer;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

/// How long to display each frame for, in milliseconds.
const FRAME_DURATION_MS: u32 = 10;

/// Display a single lit LED in the center.
const DISPLAY_CENTER: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

/// Display an arrow pointing left.
const DISPLAY_LEFT: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0],
    [1, 1, 1, 1, 1],
    [0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0],
];

/// Display an arrow pointing right.
const DISPLAY_RIGHT: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0],
    [1, 1, 1, 1, 1],
    [0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0],
];

#[entry]
fn main() -> ! {
    rtt_init_print!();

    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let mut display = Display::new(board.display_pins);

        let mut button_a = board.buttons.button_a;
        let mut button_b = board.buttons.button_b;

        loop {
            let (Ok(a_pressed), Ok(b_pressed)) =
                (button_a.is_low(), button_b.is_low());
            match (a_pressed, b_pressed) {
                // Problem doesn't specify what to do if they're both pressed.
                (true, true) => (),
                (true, false) => {
                    display.show(&mut timer, DISPLAY_LEFT, FRAME_DURATION_MS)
                }
                (false, true) => {
                    display.show(&mut timer, DISPLAY_RIGHT, FRAME_DURATION_MS)
                }
                (false, false) => {
                    display.show(&mut timer, DISPLAY_CENTER, FRAME_DURATION_MS)
                }
            }
        }
    } else {
        #[expect(clippy::panic, reason = "Can't avoid it anymore.")]
        {
            panic!()
        }
    }
}
