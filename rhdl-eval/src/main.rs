use rhdl_eval::{ice_stick_synth, testing_sim};

pub enum Modes {
    TestingSimulation,
    Synced,
}

const MODE: Modes = Modes::TestingSimulation;

fn main() {
    match MODE {
        Modes::TestingSimulation => testing_sim::simulate(),
        Modes::Synced => ice_stick_synth::synced(),
    }
}
