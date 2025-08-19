use rust_hdl::prelude::*;
use std::time::Duration;

#[derive(LogicBlock)]
/// Simple circuit to target multiple LED signals with a single pulser.
pub struct SinglePulserLEDs<const N: usize> {
    pub clock: Signal<In, Clock>,
    pub leds: Signal<Out, Bits<N>>,
    pub pulser: Pulser,
}

impl<const N: usize> SinglePulserLEDs<N> {
    pub fn new(
        clock_speed_hz: u64,
        duration_ms: u64,
        clock: Signal<In, Clock>,
        leds: Signal<Out, Bits<N>>,
    ) -> Self {
        let pulser = Pulser::new(clock_speed_hz, 1.0, Duration::from_millis(duration_ms));

        Self {
            clock,
            leds,
            pulser,
        }
    }
}

impl<const N: usize> Default for SinglePulserLEDs<N> {
    fn default() -> Self {
        let clock_speed_hz: u64 = 100_000_000;
        let pulse_rate_hz: f64 = 1.0;

        let pulser = Pulser::new(clock_speed_hz, pulse_rate_hz, Duration::from_millis(250));

        let clock = Default::default();
        let leds = Default::default();

        Self {
            clock,
            leds,
            pulser,
        }
    }
}

impl<const N: usize> Logic for SinglePulserLEDs<N> {
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

#[derive(LogicBlock)]
/// Simple circuit to target multiple LED, each with their own pulser.
pub struct MultiplePulserLEDs<const N: usize> {
    pub clock: Signal<In, Clock>,
    pub leds: Signal<Out, Bits<N>>,
    pub pulsers: [Pulser; N],
}

impl<const N: usize> MultiplePulserLEDs<N> {
    pub fn new(
        clock_speed_hz: u64,
        duration_scaling_ms: u64,
        clock: Signal<In, Clock>,
        leds: Signal<Out, Bits<N>>,
    ) -> Self {
        let pulsers = std::array::from_fn(|idx| {
            Pulser::new(
                clock_speed_hz,
                1.0,
                Duration::from_millis(((idx as u64) + 1) * duration_scaling_ms),
            )
        });

        Self {
            clock,
            leds,
            pulsers,
        }
    }
}

impl<const N: usize> Default for MultiplePulserLEDs<N> {
    fn default() -> Self {
        let clock_speed_hz: u64 = 100_000_000;
        let pulse_rate_hz: f64 = 1.0;

        let pulsers = std::array::from_fn(|idx| {
            Pulser::new(
                clock_speed_hz,
                pulse_rate_hz,
                Duration::from_millis(((idx as u64) + 1) * 100),
            )
        });

        let clock = Default::default();
        let leds = Default::default();

        Self {
            clock,
            leds,
            pulsers,
        }
    }
}

impl<const N: usize> Logic for MultiplePulserLEDs<N> {
    #[hdl_gen]
    fn update(&mut self) {
        for pulser_idx in 0..N {
            self.pulsers[pulser_idx].enable.next = true;
            self.pulsers[pulser_idx].clock.next = self.clock.val();
        }

        self.leds.next = 0b00000.into();

        for pulser_idx in 0..N {
            self.leds.next = self
                .leds
                .val()
                .replace_bit(pulser_idx, self.pulsers[pulser_idx].pulse.val());
        }
    }
}

#[derive(LogicBlock, Default)]
/// BitAdder circuit with dynamically sized input and output length (N).
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
