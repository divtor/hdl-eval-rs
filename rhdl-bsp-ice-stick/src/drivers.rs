use rhdl::prelude::*;
use rhdl_bsp::drivers::get_clock_input;

pub fn hx1ktq144_clock_driver<C: CircuitIO>(path: &Path) -> Result<Driver<C>, RHDLError> {
    // TODO implement clock driver
    let driver = hx1ktq144_clock_build();

    driver
}

pub fn hx1ktq144_leds_driver<C: CircuitIO>(path: &Path) -> Result<Driver<C>, RHDLError> {
    // TODO implement the leds_driver

    hx1ktq144_leds_build()
}

// priv
fn hx1ktq144_clock_build<C: CircuitIO>() -> Result<Driver<C>, RHDLError> {
    Ok(Default::default())
}

fn hx1ktq144_leds_build<C: CircuitIO>() -> Result<Driver<C>, RHDLError> {
    Ok(Default::default())
}
