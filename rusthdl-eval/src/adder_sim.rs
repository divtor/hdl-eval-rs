use crate::circuits::BitAdder;
use rust_hdl::core::prelude::*;
use std::num::Wrapping;

type UnsignedN = u32;
const N: usize = 32;

pub fn simulate() {
    let test_cases = (0..128)
        .map(|_| {
            let a_val = rand::random::<UnsignedN>();
            let b_val = rand::random::<UnsignedN>();
            let result = (Wrapping(a_val) + Wrapping(b_val)).0; // enables integer overflow wrapping

            (
                a_val.to_bits::<N>(),
                b_val.to_bits::<N>(),
                result.to_bits::<N>(),
            )
        })
        .collect::<Vec<_>>();

    let mut simulation: Simulation<BitAdder<N>> =
        simple_sim!(BitAdder<N>, clock, 100_000_000, ep, {
            let mut adder = ep.init()?;

            for (a, b, result) in &test_cases {
                adder.a.next = *a;
                adder.b.next = *b;

                wait_clock_cycle!(ep, clock, adder);

                println!(
                    "Test: {:x} + {:x} = {:x} (should be {:x})",
                    a,
                    b,
                    adder.result.val(),
                    result
                );

                sim_assert_eq!(ep, adder.result.val(), *result, adder);
            }

            ep.done(adder)
        });

    simulation
        .run(BitAdder::default().into(), sim_time::ONE_MILLISECOND)
        .unwrap();
}
