use crate::circuits::{MultiplePulserLEDs, SinglePulserLEDs};
use rust_hdl_bsp_ice_stick::{pins, synth};

pub fn synced_leds() {
    let duration_ms: u64 = 250;

    let ice_stick_circuit = SinglePulserLEDs::new(
        pins::CLOCK_SPEED_12MHZ,
        duration_ms,
        pins::clock_input(),
        pins::led_output(),
    );

    match synth::generate_bitstream(ice_stick_circuit, "ice_stick_synths") {
        Ok(()) => {
            println!("Generated bitstream successfully!");
        }
        Err(e) => {
            println!("Error during bitstream creation: {}", e);
        }
    }
}

pub fn asynced_leds() {
    let duration_scale_ms: u64 = 100;

    let ice_stick_circuit = MultiplePulserLEDs::new(
        pins::CLOCK_SPEED_12MHZ,
        duration_scale_ms,
        pins::clock_input(),
        pins::led_output(),
    );

    match synth::generate_bitstream(ice_stick_circuit, "ice_stick_synths") {
        Ok(()) => {
            println!("Generated bitstream successfully!");
        }
        Err(e) => {
            println!("Error during bitstream creation: {}", e);
        }
    }
}
