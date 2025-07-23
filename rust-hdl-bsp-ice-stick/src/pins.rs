use rust_hdl::core::prelude::*;

// NOTE(s):
// - Signals are abstractions of physical wires!
// - Every writer must have exactly one "driver"

/// Provides abstracted connection to the iCEstick clock
pub fn clock_input() -> Signal<In, Clock> {
    let mut clock_signal = Signal::<In, _>::default();

    // TODO
    clock_signal.add_location(0, "huh");
    clock_signal.connect();

    clock_signal
}

/// Provides abstracted connection to the iCEstick LED
pub fn led_output() -> Signal<Out, Bits<2>> {
    let mut led_signal = Signal::<Out, _>::default();

    // TODO
    led_signal.add_location(0, "huh");

    led_signal
}
