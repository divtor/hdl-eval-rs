use rusthdl_eval::{basic_sim, modes::Modes, project_sim};

// TODO:
// - choosing mode either execution parameter or via CLI
// - choosing parameters (clock speed, etc.) via CLI

const SIMULATION: Modes = Modes::BasicSimulation;

fn main() {
    match SIMULATION {
        Modes::BasicSimulation => {
            basic_sim::simulate();
        },
        Modes::ProjectSimulation => {
            project_sim::simulate();
        },
        Modes::ProjectIceStick => {}
    }
}
