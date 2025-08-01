use std::time::Duration;
use crate::circuits::MultipleLEDs;
use rust_hdl::prelude::*;

pub fn synthesize() {
    let clock_speed_hz = 1000;
    let pulse_rate_hz = 1.0;

    let pulsers = std::array::from_fn(|idx| {
        Pulser::new(
            clock_speed_hz,
            pulse_rate_hz,
            Duration::from_millis(((idx as u64) + 1) * 10),
        )
    });

    let ice_stick_circuit = MultipleLEDs {
        clock: rust_hdl_bsp_ice_stick::pins::clock_input(),
        leds: rust_hdl_bsp_ice_stick::pins::led_output(),
        pulsers
    };

    match rust_hdl_bsp_ice_stick::synth::generate_bitstream(ice_stick_circuit, "ice_stick_synths") {
        Ok(()) => {
            println!("Generated bitstream successfully!");
        },
        Err(e) => {
            println!("Error during bitstream creation: {}", e);
        }
    }
}
