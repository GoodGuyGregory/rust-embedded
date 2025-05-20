
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{
    delay::DelayNs,
    digital::{ OutputPin},
};
use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{
        pac::{self, interrupt, TIMER1},
        timer::Timer,
        gpio::Level,
        delay::Delay,
    },
};

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use critical_section_lock_mut::LockMut;

/// The display is shared by the main program and the
/// interrupt handler.
static DISPLAY: LockMut<Display<TIMER1>> = LockMut::new();

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut delay = Delay::new(board.SYST);
    let mut speaker = board.speaker_pin.into_push_pull_output(Level::Low);
    let display = Display::new(board.TIMER1, board.display_pins);

    DISPLAY.init(display);

    let mut timer2 = Timer::new(board.TIMER0);

    unsafe {
        board.NVIC.set_priority(pac::Interrupt::TIMER1, 128);
        pac::NVIC::unmask(pac::Interrupt::TIMER1);
    }

    let happy_image = GreyscaleImage::new(&[
        [0, 0, 0, 0, 0],
        [0, 9, 0, 9, 0],
        [0, 0, 0, 0, 0],
        [9, 0, 0, 0, 9],
        [0, 9, 9, 9, 0],
    ]);
    
    let drop_image = GreyscaleImage::new(&[
        [0, 0, 9, 0, 0],
        [0, 0, 9, 0, 0],
        [0, 0, 7, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 9, 0, 0],
    ]);

    loop {
        DISPLAY.with_lock(|display| display.show(&drop_image));
        timer2.delay_ms(1000u32);

        DISPLAY.with_lock(|display| display.clear());
        timer2.delay_ms(1000u32);

        DISPLAY.with_lock(|display| display.show(&happy_image));
        timer2.delay_ms(1000u32);
        

        speaker.set_high().unwrap();
        delay.delay_us(1500);
        speaker.set_low().unwrap();
        delay.delay_us(1500);


    }
}

#[interrupt]
fn TIMER1() {
    DISPLAY.with_lock(|display| display.handle_display_event());
}