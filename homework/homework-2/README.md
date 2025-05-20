# Drop Greg Witt

## What Was Developed

In this assignment I developed a a loop section to continue checking for the operations of the accelerometer. when the device is able to send accelerometer data to our loop. I am calculating the result to determine which direction in the Z axis the microbit is oriented. 

I leverage a check to determine if the device is falling, if the device is falling there is a negative direction of the `z` axis. if this occurs I am running a `display.clear` to change the display from :) to `!` as instructed.

## How Was The Development Process Went

I started by throwing together the display. designing a `!` and then the :)

after that I added the sound module from the imported onboard `board.speaker_pin` and added a push pull to toggle the noise. I was struggling to only get a small click noise on the board at first but was later able to get it to hold the tone while falling with an `if conditional check` for the changed position in the accelerometer.

## Observations


### Documentation

**For Display Logic**

[Mb2-GrayScale](https://github.com/pdx-cs-rust-embedded/mb2-grayscale)

**For Audio Logic**

[Hello Audio](https://github.com/pdx-cs-rust-embedded/hello-audio)

**For Accelerometer**  

[LSM303AGR Crate](https://crates.io/crates/lsm303agr)  
[Punch-o-meter](https://docs.rust-embedded.org/discovery-mb2/13-punch-o-meter/my-solution.html)  
[I2C Accelerometer Example](https://github.com/eldruin/driver-examples/blob/master/microbit/examples/lsm303agr-accel-mb.rs)