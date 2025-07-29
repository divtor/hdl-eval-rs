use rust_hdl::core::prelude::*;

// NOTE(s):
// - Signals are abstractions of physical wires
// - To define the pins (locations) for signals I have to use "unofficial" pad names that are defined in yosys, nextpnr, etc.
// -- I can NOT use pin number directly, as they are only accessible via the iCEstick

/// Provides abstracted connection to the iCEstick clock
pub fn clock_input() -> Signal<In, Clock> {
    let mut clock_signal = Signal::<In, _>::default();

    clock_signal.add_location(0, "21");
    clock_signal.connect();

    clock_signal
}

/// Provides abstracted connection to the iCEstick LED
pub fn led_output() -> Signal<Out, Bits<5>> {
    let mut led_signal = Signal::<Out, _>::default();

    for (idx, location) in ["99", "98", "97", "96", "95"].iter().enumerate() {
        led_signal.add_location(idx, location);
    }

    led_signal
}
