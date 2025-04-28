#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{digital::InputPin, delay::DelayNs};
#[rustfmt::skip]
use microbit::{
    board::{Board},
    display::blocking::Display,
    hal::{
        rng::Rng as HwRng,
        timer::Timer,
    },
};
use rtt_target::{rtt_init_print, rprintln};
use panic_halt as _;
use nanorand::{Rng, SeedableRng};


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
            false => *light = 0u8
        }

    }
}

// complements the current board being displayed.
// making all '1' which show light turn to '0' which would be off.
fn complement_board(light_matrix: &mut [[u8; 5]; 5]) {
    for light in light_matrix.iter_mut().flat_map(|r| r.iter_mut()) {
        if *light == 1u8 {
            *light = 0u8;
        }
        else {
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
        // // Show light_it_all for 1000ms
        life(&mut light_matrix);

        // update once per 100 ms
        display.show(&mut timer, light_matrix, 100);
        // clear the display again
        display.clear();

        // check for done. state: all display is 0:

        let a_pressed = a_button.is_low().unwrap();

        let b_pressed = b_button.is_low().unwrap();

        if a_pressed {
            rprintln!("A button pressed, checking if held:");
            randomize_board(&mut light_matrix, &mut hw_rng);
             // update once per 100 ms
            display.show(&mut timer, light_matrix, 100);
            // clear the display again
            display.clear();
        }

        if b_pressed {
            rprintln!("B button pressed");
            complement_board(&mut light_matrix);
            display.show(&mut timer, light_matrix, 100);
        
        }

        // checks if all of the cells on the display matrix are off.
        if check_lights_out(&mut light_matrix) {
            // wait 0.5 seconds then re randomize.
            timer.delay_ms(500);
            randomize_board(&mut light_matrix, &mut hw_rng );
        }

        
    }
    
}
