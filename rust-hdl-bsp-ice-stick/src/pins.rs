use rust_hdl::core::prelude::*;

/// Provides the correct iCEstick onboard oscillator frequency
pub const CLOCK_SPEED_12MHZ: u64 = 12_000_000;

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
