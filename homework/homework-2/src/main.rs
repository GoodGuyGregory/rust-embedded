#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use lsm303agr::{AccelMode, AccelOutputDataRate, AccelScale, Lsm303agr};
use microbit::{
    board::Board,
    display::nonblocking::{Display, GreyscaleImage},
    hal::{
        delay::Delay,
        gpio::Level,
        pac::{self, TIMER1, interrupt, twim0::frequency::FREQUENCY_A},
        timer::Timer,
        twim,
    },
};

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use critical_section_lock_mut::LockMut;

/// The display is shared by the main program and the
/// interrupt handler.
static DISPLAY: LockMut<Display<TIMER1>> = LockMut::new();

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut delay = Delay::new(board.SYST);
    let mut speaker = board.speaker_pin.into_push_pull_output(Level::Low);
    let display = Display::new(board.TIMER1, board.display_pins);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(&mut delay, AccelMode::Normal, AccelOutputDataRate::Hz400)
        .unwrap();

    sensor.set_accel_scale(AccelScale::G16).unwrap();

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
        DISPLAY.with_lock(|display| display.show(&happy_image));
        timer2.delay_ms(100u32);

        // determine the accelerometer is falling.
        if sensor.accel_status().unwrap().xyz_new_data() {
            let data = sensor.acceleration().unwrap();
            rprintln!(
                "Acceleration: x {} y {} z {}",
                data.x_mg(),
                data.y_mg(),
                data.z_mg()
            );

            DISPLAY.with_lock(|display| display.clear());
            timer2.delay_ms(100u32);

            DISPLAY.with_lock(|display| display.show(&drop_image));
            timer2.delay_ms(100u32);

            speaker.set_high().unwrap();
            delay.delay_us(500);
            speaker.set_low().unwrap();
            delay.delay_us(500);
        }
    }
}

#[interrupt]
fn TIMER1() {
    DISPLAY.with_lock(|display| display.handle_display_event());
}
