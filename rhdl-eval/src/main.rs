use rhdl_eval::basic_sim;

pub enum Modes {
    BasicSimulation,
    AdderSimulation,
    IceStickSynth,
}

const MODE: Modes = Modes::BasicSimulation;

fn main() {
    match MODE {
        Modes::BasicSimulation => {_ = basic_sim::simulate();},
        Modes::AdderSimulation => {},
        Modes::IceStickSynth => {},
    }
}
