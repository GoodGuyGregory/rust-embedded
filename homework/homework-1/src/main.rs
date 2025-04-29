#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
#[rustfmt::skip]
use microbit::{
    board::{Board},
    display::blocking::Display,
    hal::{
        rng::Rng as HwRng,
        timer::Timer,
    },
};
use nanorand::Rng;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

//  include the life module
mod life;
use life::*;

// iterate through the board until
fn randomize_board(light_matrix: &mut [[u8; 5]; 5], hw_rng: &mut HwRng) {
    // use into to convert the random u32 into a u128
    let mut rng = nanorand::Pcg64::new_seed(hw_rng.random_u32().into());
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map
    for light in light_matrix.iter_mut().flat_map(|r| r.iter_mut()) {
        let b: bool = rng.generate();
        match b {
            true => *light = 1u8,
            false => *light = 0u8,
        }
    }
}

// complements the current board being displayed.
// making all '1' which show light turn to '0' which would be off.
fn complement_board(light_matrix: &mut [[u8; 5]; 5]) {
    for light in light_matrix.iter_mut().flat_map(|r| r.iter_mut()) {
        if *light == 1u8 {
            *light = 0u8;
        } else {
            *light = 1u8;
        }
    }
}

// takes a mutable reference of the matrix:
// if the method determines any lights are '1' (on)
// it will return false and there is no need to start the timer.
fn check_lights_out(light_matrix: &mut [[u8; 5]; 5]) -> bool {
    for light in light_matrix.iter_mut().flat_map(|r| r.iter_mut()) {
        if *light == 1u8 {
            return false;
        }
    }
    true
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // take the board from the BSP
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    // import the hardware RNG from the microbit and create a new HWRng
    let mut hw_rng = HwRng::new(board.RNG);

    // const for frame duration
    const FRAME_DURATION: usize = 100;

    let mut a_button = board.buttons.button_a;

    let mut b_button = board.buttons.button_b;

    let mut light_matrix = [
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
    ];

    randomize_board(&mut light_matrix, &mut hw_rng);

    let mut b_button_ignore: bool = false;
    let mut ignore_frame_count: usize = 0;

    loop {
        // check for done. state: all display is 0:

        let a_pressed = a_button.is_low().unwrap();

        let b_pressed = b_button.is_low().unwrap();

        // A Button: Randomize the board display
        //-------------------
        if a_pressed {
            rprintln!("A button pressed");
            rprintln!("Randomizing Board");
            randomize_board(&mut light_matrix, &mut hw_rng);
        }
        // B Button:
        // ignore the 'B' button for 0.5s post press. to prevent interupts
        // -------------------
        else if b_pressed {
            rprintln!("B button pressed");
            b_button_ignore = true;
            rprintln!("Setting B ignore: {}", b_button_ignore)
        }
        // checks if all of the cells on the display matrix are off.
        else if check_lights_out(&mut light_matrix) {
            // wait 0.5 seconds then re randomize.
            rprintln!("Waiting for Input");
            timer.delay_ms((5 * FRAME_DURATION).try_into().unwrap());
            randomize_board(&mut light_matrix, &mut hw_rng);
        }

        // DEFAULT: Otherwise normal life steps are taken:

        rprintln!("Stepping Through Life");
        life(&mut light_matrix);

        if b_button_ignore {
            if ignore_frame_count < 4 {
                ignore_frame_count += 1;
                rprintln!("Checking Ignore Frame Count: {}", ignore_frame_count);
                complement_board(&mut light_matrix);
            } else {
                rprintln!("Switching off B Ignore:");
                b_button_ignore = false;
                rprintln!("Resetting Ignore Frame Count:");
                ignore_frame_count = 0;
            }
        }

        display.show(&mut timer, light_matrix, FRAME_DURATION.try_into().unwrap());
        // clear the display again
        display.clear();
    }
}
