use crate::fpga::led::LEDs;
use rhdl::{
    core::{ClockReset, TimedSample},
    prelude::{ClockPosEdgeExt, RunSynchronousExt, TimedStreamExt, U8, Vcd},
};
use std::str::FromStr;

fn simulation_stream() -> impl Iterator<Item = TimedSample<(ClockReset, bool)>> {
    vec![
        true, false, false, false, true, false, true, true, false, true, false,
    ]
    .into_iter()
    .with_reset(1)
    .clock_pos_edge(100)
}

pub fn simulate() {
    let leds = LEDs::<U8>::default();
    let input_stream = simulation_stream();

    let vcd = leds.run(input_stream).unwrap().collect::<Vcd>();
    let root = std::path::PathBuf::from_str("test_simulation_vcd").unwrap();

    if root.exists() {
        std::fs::remove_dir_all(&root).unwrap();
    }

    std::fs::create_dir_all(&root).unwrap();

    let _ = vcd.dump_to_file(root.join("test.vcd")).unwrap();
}
