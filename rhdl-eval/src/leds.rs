use rhdl::prelude::*;

#[derive(Clone, Debug)]
pub struct Leds<N: BitWidth> {
    leds: Bits<N>,
}

impl<N: BitWidth> Default for Leds<N> {
    fn default() -> Self {
        Self {
            leds: Default::default()
        }
    }
}

#[kernel]
pub fn blink<N: BitWidth>(cr: ClockReset, enable: bool) -> Bits<N> {
    let next_state = if enable {bits(1)} else {bits(0)};
    let next_state = if cr.reset.any() {bits(0)} else {next_state};

    next_state
}
