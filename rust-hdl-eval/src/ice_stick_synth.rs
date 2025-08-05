use crate::circuits::MultipleLEDs;
use rust_hdl::prelude::*;
use std::time::Duration;

pub fn synthesize() {
    let pulse_rate_hz: f64 = 1.0;

    let pulser = Pulser::new(
        rust_hdl_bsp_ice_stick::pins::CLOCK_SPEED_12MHZ,
        pulse_rate_hz,
        Duration::from_millis(250),
    );

    let ice_stick_circuit = MultipleLEDs {
        clock: rust_hdl_bsp_ice_stick::pins::clock_input(),
        leds: rust_hdl_bsp_ice_stick::pins::led_output(),
        pulser,
    };

    match rust_hdl_bsp_ice_stick::synth::generate_bitstream(ice_stick_circuit, "ice_stick_synths") {
        Ok(()) => {
            println!("Generated bitstream successfully!");
        }
        Err(e) => {
            println!("Error during bitstream creation: {}", e);
        }
    }
}
