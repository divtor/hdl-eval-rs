use std::time::Duration;
use rust_hdl::prelude::*;

const CLOCK_SPEED: u64 = 1_000;
const BLINKING_DURATION: u64 = 250;
const SIMULATION_RESULT_PATH: &'static str = "simulations/basic_blinker_simulation.vcd";

#[derive(LogicBlock)]
struct Blinker {
    pub clock: Signal<In, Clock>,
    pulser: Pulser,
    pub led: Signal<Out, Bit>
}

// NOTE: The update method can only contain assignments to .next or .next.field (if self is a struct!)
impl Logic for Blinker {
    #[hdl_gen]
    fn update(&mut self) {
        self.pulser.clock.next = self.clock.val();
        self.pulser.enable.next = true.into();
        self.led.next = self.pulser.pulse.val();
    }
}

impl Default for Blinker {
    fn default() -> Self {
        let clock_speed_hz: u64 = CLOCK_SPEED;
        let pulse_rate_hz: f64 = 1.0;
        let pulse_duration = Duration::from_millis(BLINKING_DURATION);

        Self { 
            clock: Default::default(), 
            pulser: Pulser::new(clock_speed_hz, pulse_rate_hz, pulse_duration), 
            led: Default::default()
        }
    }
}

pub fn simulate() {
    let mut simulation = simple_sim!(Blinker, clock, CLOCK_SPEED, ep, {
        let mut x = ep.init()?;

        wait_clock_cycles!(ep, clock, x, 4*CLOCK_SPEED);
        
        ep.done(x)
    });

    let blinker = Blinker::default();

    simulation.run_to_file(Box::new(blinker), 5 * sim_time::ONE_SEC, SIMULATION_RESULT_PATH).unwrap();
}
