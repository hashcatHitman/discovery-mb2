#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::display::blocking::Display;
use microbit::hal::Timer;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

/// The number of frames in the spinning animation.
const FRAMES: usize = 16;

/// The correct column index for the row index found at position _n_ within
/// [`crate::ACTIVE_ROW`] is found at _(n + [`crate::COLUMN_OFFSET`]) %
/// [`crate::FRAMES`]_.
const COLUMN_OFFSET: usize = 4;

/// How long to display each frame for, in milliseconds.
const FRAME_DURATION_MS: u32 = 30;

/// An empty display frame used to construct the actual frames.
const EMPTY_DISPLAY: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

/// Each value is an index used to access a row in a display frame.
/// Additionally, the corresponding column index is the value
/// [`crate::COLUMN_OFFSET`] after the current one (wrapping).
///
/// If done correctly, this gives you the indices needed to turn on the correct
/// LED for each frame of the spinning animation, assuming you are turning off
/// any other LEDs by some means.
const ACTIVE_ROW: [usize; FRAMES] =
    [0, 0, 0, 0, 0, 1, 2, 3, 4, 4, 4, 4, 4, 3, 2, 1];

#[entry]
fn main() -> ! {
    rtt_init_print!();

    if let Some(board) = Board::take() {
        let mut display = Display::new(board.display_pins);
        display.clear();

        let mut timer = Timer::new(board.TIMER0);
        let mut iteration = 0;
        loop {
            // Each frame starts as empty.
            let frame = EMPTY_DISPLAY;

            // Get a reference to the index of the row of the pixel we want to
            // turn on this frame.
            let Some(row) = ACTIVE_ROW.as_slice().get(iteration) else {
                #[expect(clippy::panic, reason = "Can't avoid it anymore.")]
                {
                    panic!()
                }
            };

            // Get the row.
            let mut frame = frame;
            match frame.get_mut(*row) {
                Some(frame_row) => {
                    // Get a reference to the index of the column of the pixel
                    // we want to turn on this frame.
                    let Some(column) = ({
                        let Some(iteration_with_offset) = iteration
                            .wrapping_add(COLUMN_OFFSET)
                            .checked_rem_euclid(FRAMES)
                        else {
                            #[expect(
                                clippy::panic,
                                reason = "Can't avoid it anymore."
                            )]
                            {
                                panic!()
                            }
                        };
                        ACTIVE_ROW.as_slice().get(iteration_with_offset)
                    }) else {
                        #[expect(
                            clippy::panic,
                            reason = "Can't avoid it anymore."
                        )]
                        {
                            panic!()
                        }
                    };

                    // Get the column within the row (AKA, the pixel), and turn
                    // it on.
                    match frame_row.get_mut(*column) {
                        Some(pixel) => *pixel = 1,
                        None => {
                            #[expect(
                                clippy::panic,
                                reason = "Can't avoid it anymore."
                            )]
                            {
                                panic!()
                            }
                        }
                    }
                }
                None => {
                    #[expect(clippy::panic, reason = "Can't avoid it anymore.")]
                    {
                        panic!()
                    }
                }
            }
            let frame = frame;

            // Display the frame and increment the iteration counter.
            display.show(&mut timer, frame, FRAME_DURATION_MS);
            #[expect(clippy::panic, reason = "Can't avoid it anymore.")]
            {
                iteration = iteration
                    .wrapping_add(1)
                    .checked_rem_euclid(FRAMES)
                    .map_or_else(|| panic!(), |success| success);
            }
        }
    } else {
        #[expect(clippy::panic, reason = "Can't avoid it anymore.")]
        {
            panic!()
        }
    }
}
