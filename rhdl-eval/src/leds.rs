use rhdl::prelude::*;

#[derive(Clone, Default, Debug, Synchronous, SynchronousDQ)]
pub struct IceStickLEDs {
    counter: rhdl_fpga::core::counter::Counter<U32>,
}

impl SynchronousIO for IceStickLEDs {
    type I = ();
    type O = b5;
    type Kernel = blink;
}

#[kernel]
pub fn blink(_cr: ClockReset, _i: (), q: Q) -> (b5, D) {
    let mut d = D::dont_care();
    
    d.counter = true;

    let output_bit = (q.counter >> 28) & 1 != 0;
    let o = if output_bit { bits(0b11110) } else { bits(0b00001) };
    
    (o, d)
}
