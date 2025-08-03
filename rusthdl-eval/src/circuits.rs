use rust_hdl::prelude::*;
use std::time::Duration;

// LED circuits -----------------------------------------------------------------------------------
#[derive(LogicBlock)]
pub struct MultipleLEDs<const N: usize> {
    pub clock: Signal<In, Clock>,
    pub leds: Signal<Out, Bits<N>>,
    pub pulser: Pulser,
}

impl<const N: usize> Default for MultipleLEDs<N> {
    fn default() -> Self {
        let clock_speed_hz: u64 = 100_000_000;
        let pulse_rate_hz: f64 = 1.0;

        let pulser = Pulser::new(
            clock_speed_hz.into(),
            pulse_rate_hz,
            Duration::from_millis(250),
        );

        let clock = Default::default();
        let leds = Default::default();

        Self {
            clock,
            leds,
            pulser,
        }
    }
}

impl<const N: usize> Logic for MultipleLEDs<N> {
    #[hdl_gen]
    fn update(&mut self) {
        self.pulser.enable.next = true;
        self.pulser.clock.next = self.clock.val();

        self.leds.next = 0b00000.into();

        if self.pulser.pulse.val() {
            self.leds.next = 0b11111.into();
        }
    }
}

// Adder circuit ----------------------------------------------------------------------------------
#[derive(LogicBlock, Default)]
pub struct BitAdder<const N: usize> {
    pub a: Signal<In, Bits<N>>,
    pub b: Signal<In, Bits<N>>,
    pub result: Signal<Out, Bits<N>>,
    pub clock: Signal<In, Clock>,
    register: DFF<Bits<N>>,
}

impl<const N: usize> Logic for BitAdder<N> {
    #[hdl_gen]
    fn update(&mut self) {
        dff_setup!(self, clock, register);
        self.register.d.next = self.a.val() + self.b.val();
        self.result.next = self.register.q.val();
    }
}
