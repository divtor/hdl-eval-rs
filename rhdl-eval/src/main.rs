use rhdl_eval::ice_stick_synth;

pub enum Modes {
    IceStickSynth,
}

const MODE: Modes = Modes::IceStickSynth;

fn main() {
    match MODE {
        Modes::IceStickSynth => {ice_stick_synth::alternating_leds();},
    }
}
