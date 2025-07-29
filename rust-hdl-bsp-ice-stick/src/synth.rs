use std::{fs::{create_dir_all, remove_dir_all, File}, path::PathBuf, str::FromStr, io::Write};

use rust_hdl::{core::prelude::*, fpga::toolchains::icestorm::generate_pcf};

// TODO: finish and test simple blinky with this
pub fn generate_bitstream<B: Block>(mut program_block: B, prefix: &str) -> std::io::Result<()> {
    program_block.connect_all();
    check_all(&program_block).unwrap();

    let dir = PathBuf::from_str(prefix).unwrap();

    let verilog_txt = generate_verilog(&program_block);
    let pcf_txt = generate_pcf(&program_block);

    remove_dir_all(&dir)?;
    create_dir_all(&dir)?;

    let mut verilog_file = File::create(dir.join("top.v")).unwrap();
    write!(verilog_file, "{}", verilog_txt)?;

    let mut pcf_file = File::create(dir.join("top.pcf")).unwrap();
    write!(pcf_file, "{}", pcf_txt)?;

    // TODO: execute yosys, nextpnr-ice40 and icepack commands

    Ok(())
}
