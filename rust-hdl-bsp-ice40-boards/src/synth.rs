//! Provides general methods to generate bitstreams or directly flash bitstreams onto iCE40-based FPGAs boards/chips.
//! Comes with wrapper methods for known boards:
//! * `iCEstick Evaluation Kit`

use rust_hdl::{core::prelude::*, fpga::toolchains::icestorm::generate_pcf};
use std::{
    fs::{File, create_dir_all, remove_dir_all},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output},
    str::FromStr,
};

use crate::chip_types::Ice40ChipType;

/// Provides the bitstream that can be flashed onto an iCE40 variant, modelled through `ChipType`.
pub fn bitstream<B: Block>(
    chip_type: Ice40ChipType,
    mut program_block: B,
    prefix: &str,
) -> std::io::Result<()> {
    program_block.connect_all();
    check_all(&program_block).unwrap();

    let dir = PathBuf::from_str(prefix).unwrap();

    let verilog_txt = generate_verilog(&program_block);
    let pcf_txt = generate_pcf(&program_block);

    remove_dir_all(&dir)?;
    create_dir_all(&dir)?;

    let mut verilog_file = File::create(dir.join("top.v")).unwrap();
    write!(verilog_file, "{verilog_txt}")?;

    let mut pcf_file = File::create(dir.join("top.pcf")).unwrap();
    write!(pcf_file, "{pcf_txt}")?;

    #[allow(clippy::suspicious_command_arg_space)]
    let output = Command::new("yosys")
        .current_dir(dir.clone())
        .arg(r#"-p  synth_ice40 -top top -json top.json"#)
        .arg("top.v")
        .output()?;

    log_out_and_err(output, &dir, "yosys_synth")?;

    let output = Command::new("nextpnr-ice40")
        .current_dir(dir.clone())
        .args([
            &format!("--{}{}", chip_type.series, chip_type.logic_capacity),
            "--package",
            chip_type.package_code,
            "--pcf",
            "top.pcf",
            "--asc",
            "top.asc",
            "--json",
            "top.json",
        ])
        .output()?;

    log_out_and_err(output, &dir, "nextpnr")?;

    let output = Command::new("icepack")
        .current_dir(dir.clone())
        .args(["top.asc", "top.bin"])
        .output()?;

    log_out_and_err(output, &dir, "icepack")?;

    Ok(())
}

pub fn flash<B: Block>(
    chip_type: Ice40ChipType,
    program_block: B,
    prefix: &str,
) -> std::io::Result<()> {
    match bitstream(chip_type, program_block, prefix) {
        Ok(()) => {
            println!("Generating bitstream during flashing was successful!");
        }
        Err(e) => {
            return Err(e);
        }
    };

    let dir = PathBuf::from_str(prefix).unwrap();

    let output = Command::new("iceprog")
        .current_dir(dir.clone())
        .arg("top.bin")
        .output()?;

    log_out_and_err(output, &dir, "iceprog")?;

    Ok(())
}

/// Wrapper synth methods for the `iCEstick Evaluation Kit`, so that no Ice40ChipType has to be defined / used.
pub mod ice_stick {
    use crate::chip_types::{self};
    use crate::synth::{self};
    use rust_hdl::prelude::Block;

    pub fn bitstream<B: Block>(program_block: B, prefix: &str) -> std::io::Result<()> {
        let chip_type = chip_types::HX1K_TQ144;

        synth::bitstream(chip_type, program_block, prefix)
    }

    pub fn flash<B: Block>(program_block: B, prefix: &str) -> std::io::Result<()> {
        let chip_type = chip_types::HX1K_TQ144;

        synth::flash(chip_type, program_block, prefix)
    }
}

/// Provides logging of the executed command outputs
fn log_out_and_err(output: Output, dir: &Path, basename: &str) -> Result<(), std::io::Error> {
    let (stdout, stderr) = (
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    );

    let mut out_file = File::create(dir.join(format!("{basename}.out")))?;
    write!(out_file, "{stdout}")?;

    let mut err_file = File::create(dir.join(format!("{basename}.err")))?;
    write!(err_file, "{stderr}")?;

    Ok(())
}
