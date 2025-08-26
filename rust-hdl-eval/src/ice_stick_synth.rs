use crate::fpga;
use rust_hdl::prelude::Block;
use rust_hdl_bsp_ice40_boards::{pins, synth};

/// Flashes a `SinglePulserLEDs` block onto the `iCEstick Evaluation Kit`
pub fn synced_leds(duration_ms: u64) {
    let single_pulser_leds = fpga::leds::SinglePulserLEDs::new(
        pins::ice_stick::CLOCK_SPEED_12MHZ,
        duration_ms,
        pins::ice_stick::clock(),
        pins::ice_stick::leds(),
    );

    synth(single_pulser_leds);
}

/// Flashes a `MultiplePulserLEDs` block onto the `iCEstick Evaluation Kit`
pub fn asynced_leds(duration_scale_ms: u64) {
    let mult_pulser_leds = fpga::leds::MultiplePulserLEDs::new(
        pins::ice_stick::CLOCK_SPEED_12MHZ,
        duration_scale_ms,
        pins::ice_stick::clock(),
        pins::ice_stick::leds(),
    );

    synth(mult_pulser_leds);
}

/// Flashes any suitable program block into the `iCEstick Evaluation Kit`
fn synth<B: Block>(program_block: B) {
    let dir_name = "ice_stick_synths";

    // NOTE: Failure of 'icestorm' commands does not seem to create an Error instance
    // -> errors get logged into log files.
    // This means that the program won't actually fail, even if the iCEstick is not plugged in
    match synth::ice_stick::flash(program_block, dir_name) {
        Ok(()) => {
            println!(
                "Flashing process finished; If flashing failed verify command logs in .out and .err files!"
            );
        }
        Err(e) => {
            println!("Error during bitstream flashing: {e}");
        }
    }
}
