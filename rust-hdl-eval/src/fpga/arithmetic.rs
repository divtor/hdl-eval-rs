use rust_hdl::prelude::*;

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
