use rusthdl_eval::{basic_sim, testing_sim};

// TODO:
// - choosing mode either execution parameter or via CLI
// - choosing parameters (clock speed, etc.) via CLI

pub enum Modes {
    BasicSimulation,
    TestingSimulation,
    AdderSimulation,
    IceStickSynth,
}

const SIMULATION: Modes = Modes::TestingSimulation;

fn main() {
    match SIMULATION {
        Modes::BasicSimulation => {
            basic_sim::simulate();
        }
        Modes::TestingSimulation => {
            testing_sim::simulate();
        }
        Modes::AdderSimulation => {}
        Modes::IceStickSynth => {}
    }
}
