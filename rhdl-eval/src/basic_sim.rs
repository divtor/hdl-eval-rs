// TODO write my own program, just trying to get something to work currently

use rhdl::prelude::*;
use rhdl_fpga::doc::write_svg_as_markdown;
use rhdl_fpga::dsp::lerp::fixed::lerp_unsigned;

#[derive(PartialEq, Digital)]
pub struct LerpIn<N, M>
where
    N: BitWidth,
    M: BitWidth,
{
    pub lower_value: Bits<N>,
    pub upper_value: Bits<N>,
    pub factor: Bits<M>,
}

// An wrapper function to call the `lerp_unsigned`
#[kernel]
pub fn wrap_lerp<N, M>(_cr: ClockReset, i: LerpIn<N, M>) -> Bits<N>
where
    N: BitWidth,
    M: BitWidth,
{
    lerp_unsigned::<N, M>(i.lower_value, i.upper_value, i.factor)
}

pub fn simulate() -> Result<(), RHDLError> {
    let uut: Func<LerpIn<U8, U4>, Bits<U8>> = Func::try_new::<wrap_lerp<U8, U4>>()?;
    
    // Simulate a ramp
    let ramp = (0..15)
        .map(|x| LerpIn {
            upper_value: bits(255),
            lower_value: bits(0),
            factor: bits(x),
        })
        .without_reset()
        .clock_pos_edge(100);

    let vcd = uut.run(ramp)?.collect::<Vcd>();
    
    write_svg_as_markdown(vcd, "lerp.md", SvgOptions::default())?;

    Ok(())
}
