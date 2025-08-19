use rust_hdl_eval::{adder_sim, basic_sim, ice_stick_synth, testing_sim};

const HELP_HELP: &str = "--help: Get information about available commands (was executed here)!";
const HELP_MODE: &str = "--mode [Mode]: [Mode] is either [\"Basic\" | \"Testing\" | \"Adder\"] for simulations or [\"Synced\" | \"Asynced\"] for loading programs onto the iCEstick Evaluation Kit";
const HELP_DURATION: &str ="--duration [ms]: Sets the blinking duration for the \"Synced\" mode (defaults to 250 ms)!";
const HELP_DURATION_SCALE: &str = "--duration_scale [ms]: Sets the blinking duration scaling for the \"Asynced\" mode (defaults to 100 ms)!";

pub enum Modes {
    BasicSimulation,
    TestingSimulation,
    AdderSimulation,
    LedsSynced,
    LedsAsynced,
}

fn main() {
    let cli_arguments: Vec<String> = std::env::args().collect();

    let mut mode = Modes::TestingSimulation;
    let mut duration_ms: u64 = 250;
    let mut duration_scale_ms: u64 = 100;

    if !cli_arguments.is_empty() {
        if cli_arguments.contains(&"--help".to_string()) {
            println!("Available commands:");
            println!("{HELP_HELP}");
            println!("{HELP_MODE}");
            println!("{HELP_DURATION}");
            println!("{HELP_DURATION_SCALE}");
        }

        for (idx, argument) in cli_arguments
            .iter()
            .filter(|a| a.as_str() != "--help")
            .enumerate()
        {
            match argument.as_str() {
                "--mode" => match cli_arguments.get(idx + 1).map(|m| m.as_str()) {
                    Some(m) => match m {
                        "Basic" => {
                            mode = Modes::BasicSimulation;
                        }
                        "Testing" => {
                            mode = Modes::TestingSimulation;
                        }
                        "Adder" => {
                            mode = Modes::AdderSimulation;
                        }
                        "Synced" => {
                            mode = Modes::LedsSynced;
                        }
                        "Asynced" => {
                            mode = Modes::LedsAsynced;
                        }
                        _ => panic!("Unkown mode {m}"),
                    },
                    None => panic!("Mode value missing!"),
                },
                "--duration" => match cli_arguments.get(idx + 1) {
                    Some(ms) => {
                        duration_ms = match ms.parse::<u64>() {
                            Ok(duration) => duration,
                            Err(_) => panic!("Failed to parse duration {ms} to u64"),
                        }
                    }
                    None => panic!("Duration value is missing!"),
                },
                "--duration_scale" => match cli_arguments.get(idx + 1) {
                    Some(ms) => {
                        duration_scale_ms = match ms.parse::<u64>() {
                            Ok(duration) => duration,
                            Err(_) => panic!("Failed to parse duration_scale {ms} to u64"),
                        }
                    }
                    None => panic!("Duration scaling value is missing!"),
                },
                _ => {
                    if argument.starts_with("--") {
                        println!("Unkown argument {argument}");
                    }
                }
            }
        }
    }

    match mode {
        Modes::BasicSimulation => {
            println!("Executing basic simulation...");
            basic_sim::simulate();
        }
        Modes::TestingSimulation => {
            println!("Executing testing simulation...");
            testing_sim::simulate();
        }
        Modes::AdderSimulation => {
            println!("Executing bit adder simulation...");
            adder_sim::simulate();
        }
        Modes::LedsSynced => {
            println!(
                "Flashing synced blinker onto `iCE Evaluation Kit` with an {duration_ms} ms interval..."
            );
            ice_stick_synth::synced_leds(duration_ms);
        }
        Modes::LedsAsynced => {
            println!(
                "Flashing asynced blinker onto `iCE Evaluation Kit` with an {duration_scale_ms} ms scaling interval..."
            );
            ice_stick_synth::asynced_leds(duration_scale_ms);
        }
    }
}
