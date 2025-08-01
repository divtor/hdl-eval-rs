use rust_hdl::prelude::*;
use std::{array, time::Duration};

use crate::circuits::MultipleLEDs;

pub fn simulate() {
    // TODO properly understand clock settings and timing and document it
    // Current configuration: simulation CLK: 100_000; wait 10_000 clock cycles; Pulser clock speed hz: 1000; pulse rate hz: 1.0
    // 100 ms simulation length,  10 ms cycles (of the logic being applied)
    // how to keep same length and reduce 'cycle length'?
    // the parameters are still cryptic; TODO: dig through doc and find the real purpose of the values
    // maybe do not use simple_sim! macro, to gain better control

    let simulation_max_time = sim_time::ONE_MILLISECOND * 10_000;
    let simulation_vcd_file_path = "simulations/project_simulation.vcd";

    // CIRCUIT CREATION ---------------------------------------------------------------------------
    let clock_speed_hz: u64 = 1000;
    let pulse_rate_hz: f64 = 1.0;

    let pulsers = array::from_fn(|idx| {
        Pulser::new(
            clock_speed_hz,
            pulse_rate_hz,
            Duration::from_millis(((idx as u64) + 1) * 10),
        )
    });

    let clock = Default::default();
    let leds = Default::default();

    let led_simulation_circuit = MultipleLEDs {
        clock,
        leds,
        pulsers,
    };

    // SIMULATE -----------------------------------------------------------------------------------
    let mut simulation = Simulation::new();

    simulation.add_clock(1_000_000, |circuit: &mut Box<MultipleLEDs<8>>| {
        circuit.clock.next = !circuit.clock.val()
    });

    simulation.add_testbench(move |mut fixture: Sim<MultipleLEDs<8>>| {
        let mut circuit = fixture.init()?;
        wait_clock_cycles!(fixture, clock, circuit, 10 * 1000);
        fixture.done(circuit)
    });

    simulation
        .run_to_file(
            Box::new(led_simulation_circuit),
            simulation_max_time,
            simulation_vcd_file_path,
        )
        .unwrap();
}
