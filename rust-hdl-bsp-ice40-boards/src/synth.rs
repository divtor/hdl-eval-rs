//! Provides general methods to generate bitstreams or directly flash bitstreams onto iCE40-based FPGAs boards/chips.
//! Comes with wrapper methods for known boards:
//! * `iCEstick Evaluation Kit`

use crate::chip_types::Ice40ChipType;
use rust_hdl::{
    core::{check_error::CheckError, prelude::*},
    fpga::toolchains::icestorm::generate_pcf,
};
use std::{fs, io::Write, path, process, str::FromStr};

/// Provides the bitstream that can be flashed onto an iCE40 variant, modelled through `ChipType`.
/// Individual commands can fail and their error will be written into their `.err` files.
pub fn bitstream<B: Block>(
    chip_type: Ice40ChipType,
    mut program_block: B,
    prefix: &str,
) -> std::io::Result<()> {
    // initial check and setup --------------------------------------------------------------------
    program_block.connect_all();
    match check_all(&program_block) {
        Err(err) => match err {
            CheckError::LogicLoops(_) => panic!("Logic loops in program block!"),
            CheckError::OpenSignal(_) => panic!("Open signal in program block!"),
            CheckError::WritesToInputs(_) => panic!("Program block tries to write to inputs!"),
        },
        _ => {
            // do nothing
        }
    }

    let dir = match path::PathBuf::from_str(prefix) {
        Ok(path_buf) => path_buf,
        _ => panic!("Infallible action failed!"),
    };

    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }

    fs::create_dir_all(&dir)?;

    // bitstream generation starts ----------------------------------------------------------------
    let mut verilog_file = fs::File::create(dir.join("top.v")).unwrap();
    write!(verilog_file, "{}", generate_verilog(&program_block))?;

    let mut pcf_file = fs::File::create(dir.join("top.pcf")).unwrap();
    write!(pcf_file, "{}", generate_pcf(&program_block))?;

    #[allow(clippy::suspicious_command_arg_space)]
    let output = process::Command::new("yosys")
        .current_dir(dir.clone())
        .arg(r#"-p  synth_ice40 -top top -json top.json"#)
        .arg("top.v")
        .output()?;

    log_out_and_err(output, &dir, "yosys_synth")?;

    let output = process::Command::new("nextpnr-ice40")
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

    let output = process::Command::new("icepack")
        .current_dir(dir.clone())
        .args(["top.asc", "top.bin"])
        .output()?;

    log_out_and_err(output, &dir, "icepack")?;

    Ok(())
}

/// Generates and flashes a bitstream onto an iCE40 variant, modelled through `ChipType`.
/// `iceprog` command might fail and the error will be written into the `.err` files, instead of firing it in code.
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

    let dir = path::PathBuf::from_str(prefix).unwrap();

    let output = process::Command::new("iceprog")
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
fn log_out_and_err(
    output: process::Output,
    dir: &path::Path,
    basename: &str,
) -> Result<(), std::io::Error> {
    let (stdout, stderr) = (
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    );

    let mut out_file = fs::File::create(dir.join(format!("{basename}.out")))?;
    write!(out_file, "{stdout}")?;

    let mut err_file = fs::File::create(dir.join(format!("{basename}.err")))?;
    write!(err_file, "{stderr}")?;

    Ok(())
}
