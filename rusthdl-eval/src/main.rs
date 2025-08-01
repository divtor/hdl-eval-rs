use rusthdl_eval::{adder_sim, basic_sim, ice_stick_synth, testing_sim};

// TODO:
// - choosing mode either execution parameter or via CLI
// - choosing parameters (clock speed, etc.) via CLI

const HELP_HELP: &'static str = "--help: Get information about available commands (was executed here)!";
const HELP_MODE: &'static str = "--mode [Mode]: [Mode] is either [\"Basic\" | \"Testing\" | \"Adder\"] for simulations or [\"IceStick\"] for loading programs onto a iCE40HX1KTQ144 based iCEstick";

pub enum Modes {
    BasicSimulation,
    TestingSimulation,
    AdderSimulation,
    IceStickSynth,
}

const SIMULATION: Modes = Modes::TestingSimulation;

fn main() {
    let cli_arguments: Vec<String> = std::env::args().collect();

    if cli_arguments.len() == 0 {
        match SIMULATION {
            Modes::BasicSimulation => basic_sim::simulate(),
            Modes::TestingSimulation => testing_sim::simulate(),
            Modes::AdderSimulation => adder_sim::simulate(),
            Modes::IceStickSynth => ice_stick_synth::synthesize()
        }
    } else {
        // TODO: make this better and let user set dynamic values for simulations (e.g. clock_hz)
        // move this into another lib when implementing properly
        if cli_arguments.contains(&"--help".to_string()) {
            println!("Available commands:");
            println!("{HELP_HELP}");
            println!("{HELP_MODE}");
        }

        for (idx, argument) in cli_arguments.iter().filter(|a| a.as_str() != "--help").enumerate() {
            match argument.as_str() {
                "--mode" => {
                    match cli_arguments.get(idx+1).map(|m| m.as_str()) {
                        Some("Basic") => basic_sim::simulate(),
                        Some("Testing") => testing_sim::simulate(),
                        Some("Adder") => adder_sim::simulate(),
                        Some("IceStick") => ice_stick_synth::synthesize(),
                        _ => println!("Mode value missing!")
                    }
                }
                _ => {
                    if argument.as_str().starts_with("--") {
                        println!("Unkown argument {}", argument);
                    }
                }
            }
        }
    }
}
