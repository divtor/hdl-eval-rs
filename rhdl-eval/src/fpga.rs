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
    let next_count: Bits<N> = if enable { bits(1) } else { bits(0) };
    (q.leds, D::<N> { leds: next_count })
}
