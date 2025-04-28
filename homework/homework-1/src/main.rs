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
// use nanorand::{Rng, SeedableRng};


//  include the life module
// mod life;
// use life::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // take the board from the BSP
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut a_button = board.buttons.button_a;

    let mut b_button = board.buttons.button_b;


    let light_it_all = [
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
    ];

    loop {
        // Show light_it_all for 1000ms
        display.show(&mut timer, light_it_all, 1000);
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
