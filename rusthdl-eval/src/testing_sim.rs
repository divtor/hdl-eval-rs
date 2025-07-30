use std::{array, time::Duration};

use rust_hdl::prelude::*;

const CLOCK_SPEED: u64 = 1_000;
const SIMULATION_RESULT_PATH: &'static str = "simulations/project_simulation.vcd";

#[derive(LogicBlock)]
pub struct MultipleLEDSim<const N: usize> {
    pub clock: Signal<In, Clock>,
    pub leds: Signal<Out, Bits<N>>,
    pulsers: [Pulser; N],
}

impl<const N: usize> Default for MultipleLEDSim<N> {
    fn default() -> Self {
        let clock_speed_hz: u64 = CLOCK_SPEED;
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

        Self {
            clock,
            leds,
            pulsers,
        }
    }
}

impl<const N: usize> Logic for MultipleLEDSim<N> {
    #[hdl_gen]
    fn update(&mut self) {
        for pulser_idx in 0..N {
            self.pulsers[pulser_idx].clock.next = self.clock.val();
            self.pulsers[pulser_idx].enable.next = true.into();
        }

        self.leds.next = 0.into();

        for pulser_idx in 0..N {
            self.leds.next = self
                .leds
                .val()
                .replace_bit(pulser_idx, self.pulsers[pulser_idx].pulse.val());
        }
    }
}

pub fn simulate() {
    let simulation_clock_speed = CLOCK_SPEED;
    let simulation_max_time = sim_time::ONE_MILLISECOND * 10_000;
    let simulation_vcd_file_path = SIMULATION_RESULT_PATH;

    // TODO create MultipleLEDSim instance here instead of using default
    // TODO properly understand clock settings and timing and document it

    let led_simulation_circuit = MultipleLEDSim::default();

    let mut simulation = simple_sim!(
        MultipleLEDSim<8>, // module instance
        clock, // clock field of the module instance
        simulation_clock_speed, 
        endpoint, 
        {
            let mut circuit = endpoint.init()?;
            wait_clock_cycles!(endpoint, clock, circuit, 100);
            endpoint.done(circuit)
        }
    );

    simulation
        .run_to_file(
            Box::new(led_simulation_circuit),
            simulation_max_time,
            simulation_vcd_file_path,
        )
        .unwrap();
}
