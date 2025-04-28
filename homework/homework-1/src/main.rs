#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
#[rustfmt::skip]
use microbit::{
    board::{Board},
    display::blocking::Display,
    hal::{
        // Rng as HwRng,
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
fn randomize_board(light_matrix: &mut [[u8; 5]; 5]) {

    let mut rng = nanorand::Pcg64::new_seed(31);
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map
    for light in light_matrix.iter_mut().flat_map(|r| r.iter_mut()) {
        let b: bool = rng.generate();
        rprintln!("generated: {}", b);
        match b {
            true => *light = 1u8,
            false => *light = 0u8
        }

    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // take the board from the BSP
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut a_button = board.buttons.button_a;

    let mut b_button = board.buttons.button_b;

    let mut light_matrix = [
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
    ];

    randomize_board(&mut light_matrix);

    rprintln!("{:?}",light_matrix);

    loop {
        // // Show light_it_all for 1000ms
        life(&mut light_matrix);
        

        display.show(&mut timer, light_matrix, 100);
        // clear the display again
        display.clear();

        let a_pressed = a_button.is_low().unwrap();

        let b_pressed = b_button.is_low().unwrap();

        if a_pressed {
            rprintln!("A button pressed");
        }

        if b_pressed {
            rprintln!("B button pressed");
        }
        
    }
    
}
