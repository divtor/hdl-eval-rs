use rust_hdl::prelude::*;
use std::{array, time::Duration};

// LED circuits -----------------------------------------------------------------------------------
#[derive(LogicBlock)]
pub struct MultipleLEDs<const N: usize> {
    pub clock: Signal<In, Clock>,
    pub leds: Signal<Out, Bits<N>>,
    pub pulsers: [Pulser; N],
}

impl<const N: usize> Default for MultipleLEDs<N> {
    fn default() -> Self {
        let clock_speed_hz: u64 = 1000;
        let pulse_rate_hz: f64 = 1.0;

        let pulsers = array::from_fn(|idx| {
            Pulser::new(
                clock_speed_hz,
                pulse_rate_hz,
                Duration::from_millis(((idx as u64) + 1) * 10),
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

impl<const N: usize> Logic for MultipleLEDs<N> {
    #[hdl_gen]
    fn update(&mut self) {
        for pulser_idx in 0..N {
            self.pulsers[pulser_idx].clock.next = self.clock.val();
            self.pulsers[pulser_idx].enable.next = true.into();
        }

        self.leds.next = 0.into();

        for pulser_idx in 0..N {
            self.leds.next = self
                .leds
                .val()
                .replace_bit(pulser_idx, self.pulsers[pulser_idx].pulse.val());
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
    register: DFF<Bits<N>>
}

impl<const N: usize> Logic for BitAdder<N> {
    #[hdl_gen]
    fn update(&mut self) {
        dff_setup!(self, clock, register);
        self.register.d.next = self.a.val() + self.b.val();
        self.result.next = self.register.q.val();
    }
}

