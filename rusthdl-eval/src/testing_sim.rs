use rust_hdl::prelude::*;
use std::time::Duration;

use crate::circuits::MultipleLEDs;

pub fn simulate() {
    let simulation_max_time = sim_time::ONE_SEC * 4;
    let simulation_vcd_file_path = "simulations/project_simulation.vcd";

    // CIRCUIT CREATION ---------------------------------------------------------------------------
    let clock_speed_hz: u64 = 10_000;
    let pulse_rate_hz: f64 = 1.0;

    let pulser = Pulser::new(clock_speed_hz.into(), pulse_rate_hz, Duration::from_millis(250));

    let clock = Default::default();
    let leds = Default::default();

    let led_simulation_circuit = MultipleLEDs {
        clock,
        leds,
        pulser,
    };

    // SIMULATE -----------------------------------------------------------------------------------
    let mut simulation = Simulation::new();

    simulation.add_clock(clock_speed_hz, |circuit: &mut Box<MultipleLEDs<8>>| {
        circuit.clock.next = !circuit.clock.val()
    });

    simulation.add_testbench(move |mut fixture: Sim<MultipleLEDs<8>>| {
        let mut circuit = fixture.init()?;
        wait_clock_cycles!(fixture, clock, circuit, clock_speed_hz * 4);
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
