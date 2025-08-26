use std::str::FromStr;

use crate::fpga::leds::SinglePulserLEDs;
use rust_hdl::prelude::*;

pub fn simulate() {
    let simulation_max_time = sim_time::ONE_SEC * 4;

    let dir = std::path::PathBuf::from_str("simulations").unwrap();

    if !dir.exists() {
        _ = std::fs::create_dir_all(dir);
    }

    let simulation_vcd_file_path = "simulations/project_simulation.vcd";

    // CIRCUIT CREATION ---------------------------------------------------------------------------
    let clock_speed_hz: u64 = 10_000;
    let duration_ms = 250;
    let clock = Default::default();
    let leds = Default::default();

    let led_simulation_circuit = SinglePulserLEDs::new(clock_speed_hz, duration_ms, clock, leds);

    // SIMULATE -----------------------------------------------------------------------------------
    let mut simulation = Simulation::new();

    simulation.add_clock(clock_speed_hz, |circuit: &mut Box<SinglePulserLEDs<8>>| {
        circuit.clock.next = !circuit.clock.val()
    });

    simulation.add_testbench(move |mut fixture: Sim<SinglePulserLEDs<8>>| {
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
