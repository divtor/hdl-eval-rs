use rhdl::prelude::*;

// TODO: evaluate how to actually implement this efficiently
// nothing is documented yet, so this might be a more cumbersome as initially expected

pub fn hx1ktq144_bitstream<B: Synchronous + SynchronousDQ>(
    program_block: B,
    _prefix: &str,
) -> std::io::Result<()> {
    let adapter: Adapter<B, Red> = Adapter::new(program_block); // what even is Red here? LED color?
    let _fixture = Fixture::new("top", adapter);

    // TODO 1: write drivers to fixture for clock and leds and setup fixture
    // TODO 2: write builder for ice_stick and implement here

    Ok(())
}

pub fn hx1ktq144_flash<B: Synchronous + SynchronousDQ>(
    program_block: B,
    prefix: &str,
) -> std::io::Result<()> {
    match hx1ktq144_bitstream(program_block, prefix) {
        Ok(()) => {
            println!("Generated bitream successfulyl during flashing!");
        }
        Err(e) => {
            return Err(e);
        }
    }

    Ok(())
}
