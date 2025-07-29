use rusthdl_eval::{basic_sim, project_sim};

// TODO:
// - choosing mode either execution parameter or via CLI
// - choosing parameters (clock speed, etc.) via CLI

pub enum Modes {
    BasicSimulation,
    ProjectSimulation,
    ProjectIceStick
}

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
