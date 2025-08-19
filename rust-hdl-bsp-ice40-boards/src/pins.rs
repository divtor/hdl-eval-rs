//! Provides utility functions regarding the pins of iCE40 based FPGA boards.
//! Currently implemented:
//! * `iCEstick Evaluation Kit`

/// Provides utility functions regarding the pins of the `iCEstick Evaluation Kit`.
pub mod ice_stick {
    use rust_hdl::core::prelude::*;

    /// Provides the correct iCEstick onboard oscillator frequency
    pub const CLOCK_SPEED_12MHZ: u64 = 12_000_000;

    /// The iCEstick clock pin as an abstracted Signal
    pub fn clock() -> Signal<In, Clock> {
        let mut clock = Signal::<In, _>::default();

        clock.add_location(0, "21");
        clock.connect();

        clock
    }

    /// The iCEstick led pins as an abstracted Signal
    pub fn leds() -> Signal<Out, Bits<5>> {
        let mut leds = Signal::<Out, _>::default();

        for (idx, location) in ["99", "98", "97", "96", "95"].iter().enumerate() {
            leds.add_location(idx, location);
        }

        leds
    }
}
