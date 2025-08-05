use crate::circuits::{MultiplePulserLEDs, SinglePulserLEDs};
use rust_hdl::prelude::Block;
use rust_hdl_bsp_ice_stick::{pins, synth};

/// Flashes a `SinglePulserLEDs` block onto the HX1KTQ144 iCEstick
pub fn synced_leds() {
    let duration_ms: u64 = 250;

    let single_pulser_leds = SinglePulserLEDs::new(
        pins::CLOCK_SPEED_12MHZ,
        duration_ms,
        pins::hx1ktq144_clock(),
        pins::hx1ktq144_leds(),
    );

    synth(single_pulser_leds);
}

/// Flashes a `MultiplePulserLEDs` block onto the HX1KTQ144 iCEstick
pub fn asynced_leds() {
    let duration_scale_ms: u64 = 100;

    let mult_pulser_leds = MultiplePulserLEDs::new(
        pins::CLOCK_SPEED_12MHZ,
        duration_scale_ms,
        pins::hx1ktq144_clock(),
        pins::hx1ktq144_leds(),
    );

    synth(mult_pulser_leds);
}

/// Flashes any suitable program block into the HX1KTQ144 iCEstick
fn synth<B: Block>(program_block: B) {
    let dir_name = "ice_stick_synths";

    // NOTE: Failure of 'icestorm' commands does not seem to create an Error instance
    // This means that the program won't actually fail, even if the iCEstick is not plugged in
    match synth::hx1ktq144_flash(program_block, dir_name) {
        Ok(()) => {
            println!("Flashed the bitstream sucessfully!");
        }
        Err(e) => {
            println!("Error during bitstream flashing: {}", e);
        }
    }
}
