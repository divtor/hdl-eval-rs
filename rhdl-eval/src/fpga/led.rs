#[allow(unused_imports)]
use std::str::FromStr; // necessary to call FromStr for PathBuf in test method

use rhdl::prelude::*;
use rhdl_fpga::core::dff;

#[derive(Clone, Debug, Synchronous, SynchronousDQ)]
/// Generic circuit that causes multiple LEDs to blink in unison
pub struct LEDs<N: BitWidth> {
    leds: dff::DFF<Bits<N>>,
}

impl<N: BitWidth> Default for LEDs<N> {
    fn default() -> Self {
        Self {
            leds: dff::DFF::new(Bits::<N>::default()),
        }
    }
}

impl<N: BitWidth> SynchronousIO for LEDs<N> {
    type I = bool;
    type O = Bits<N>;
    type Kernel = blink<N>;
}

#[kernel]
/// Blinky kernel function
pub fn blink<N: BitWidth>(_cr: ClockReset, enable: bool, q: Q<N>) -> (Bits<N>, D<N>) {
    let next_count: Bits<N> = if enable {
        bits(N::BITS as u128)
    } else {
        bits(0)
    };
    (q.leds, D::<N> { leds: next_count })
}

pub fn test_input_stream() -> impl Iterator<Item = TimedSample<(ClockReset, bool)>> {
    vec![true, false, true, false, true, false]
        .into_iter()
        .with_reset(1)
        .clock_pos_edge(100)
}

#[test]
fn test_vcd_trace() {
    let leds = LEDs::<U5>::default();
    let input_stream = test_input_stream();

    let vcd = leds.run(input_stream).unwrap().collect::<Vcd>();
    let root = std::path::PathBuf::from_str("test_vcd").unwrap();

    if root.exists() {
        std::fs::remove_dir_all(&root).unwrap();
    }

    std::fs::create_dir_all(&root).unwrap();

    let _ = vcd.dump_to_file(root.join("test.vcd")).unwrap();

    assert_eq!(1, 1);
}
