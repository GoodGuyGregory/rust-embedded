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

// Iterate through the board and randomize the values
fn randomize_board(light_matrix: &mut [[u8; 5]; 5], hw_rng: &mut HwRng) {
    let mut rng = nanorand::Pcg64::new_seed(hw_rng.random_u32().into());
    for light in light_matrix.iter_mut().flat_map(|r| r.iter_mut()) {
        let b: bool = rng.generate();
        *light = if b { 1u8 } else { 0u8 };
    }
}

// Complements the current board being displayed
fn complement_board(light_matrix: &mut [[u8; 5]; 5]) {
    for light in light_matrix.iter_mut().flat_map(|r| r.iter_mut()) {
        *light = if *light == 1u8 { 0u8 } else { 1u8 };
    }
}

// Checks if all lights are off
fn check_lights_out(light_matrix: &[[u8; 5]; 5]) -> bool {
    light_matrix.iter().flatten().all(|&light| light == 0u8)
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Take the board from the BSP
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut hw_rng = HwRng::new(board.RNG);

    const FRAME_DURATION: u16 = 100;

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

    loop {
        let a_pressed = a_button.is_low().unwrap();
        let b_pressed = b_button.is_low().unwrap();

        // A Button: Randomize the board display
        if a_pressed {
            rprintln!("A button pressed");
            rprintln!("Randomizing Board");
            randomize_board(&mut light_matrix, &mut hw_rng);
        }
        // B Button: Ignore the 'B' button for 0.5s post press
        else if b_pressed {
            rprintln!("B button pressed");

            complement_board(&mut light_matrix);

            display.show(&mut timer, light_matrix, FRAME_DURATION.into());

            for _ in 0..4 {
                
                rprintln!("Step through Complemented Life");
                life(&mut light_matrix);

                complement_board(&mut light_matrix);

                display.show(&mut timer, light_matrix, FRAME_DURATION.into());

            }
        }
        // Check if all lights are out
        else if check_lights_out(&light_matrix) {
            rprintln!("Waiting for Input");
            timer.delay_ms(FRAME_DURATION.into());
            randomize_board(&mut light_matrix, &mut hw_rng);
        }

        // Step through life
        rprintln!("Stepping Through Life");
        life(&mut light_matrix);

        display.show(&mut timer, light_matrix, FRAME_DURATION.into());
    }
}
