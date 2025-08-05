use rust_hdl_eval::{adder_sim, basic_sim, ice_stick_synth, testing_sim};

const HELP_HELP: &'static str =
    "--help: Get information about available commands (was executed here)!";
const HELP_MODE: &'static str = "--mode [Mode]: [Mode] is either [\"Basic\" | \"Testing\" | \"Adder\"] for simulations or [\"Synced\" | \"Asynced\"] for loading programs onto a iCE40HX1KTQ144 based iCEstick";

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
            Modes::IceStickSynth => ice_stick_synth::synced_leds(),
        }
    } else {
        if cli_arguments.contains(&"--help".to_string()) {
            println!("Available commands:");
            println!("{HELP_HELP}");
            println!("{HELP_MODE}");
        }

        for (idx, argument) in cli_arguments
            .iter()
            .filter(|a| a.as_str() != "--help")
            .enumerate()
        {
            match argument.as_str() {
                "--mode" => match cli_arguments.get(idx + 1).map(|m| m.as_str()) {
                    Some(mode) => match mode {
                        "Basic" => basic_sim::simulate(),
                        "Testing" => testing_sim::simulate(),
                        "Adder" => adder_sim::simulate(),
                        "Synced" => ice_stick_synth::synced_leds(),
                        "Asynced" => ice_stick_synth::asynced_leds(),
                        _ => println!("Unkown mode {}", mode),
                    },
                    _ => println!("Mode value missing!"),
                },
                _ => {
                    if argument.as_str().starts_with("--") {
                        println!("Unkown argument {}", argument);
                    }
                }
            }
        }
    }
}
