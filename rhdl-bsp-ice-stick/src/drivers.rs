use rhdl::prelude::*;

pub fn clock_driver<C: CircuitIO>(_path: &Path) -> Result<Driver<C>, RHDLError> {
    // TODO implement clock driver
    let driver = clock_build();

    driver
}

pub fn leds_driver<C: CircuitIO>(_path: &Path) -> Result<Driver<C>, RHDLError> {
    // TODO implement the leds_driver

    leds_build()
}

// priv
fn clock_build<C: CircuitIO>() -> Result<Driver<C>, RHDLError> {
    Ok(Default::default())
}

fn leds_build<C: CircuitIO>() -> Result<Driver<C>, RHDLError> {
    Ok(Default::default())
}
