use rhdl_eval::{ice_stick_synth, sim};

pub enum Modes {
    TestingSimulation,
    Synced,
}

const MODE: Modes = Modes::TestingSimulation;

fn main() {
    match MODE {
        Modes::TestingSimulation => sim::testing_sim::simulate(),
        Modes::Synced => ice_stick_synth::synced(),
    }
}
