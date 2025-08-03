use rust_hdl::{core::prelude::*, fpga::toolchains::icestorm::generate_pcf};
use std::{
    fs::{File, create_dir_all, remove_dir_all},
    io::Write,
    path::PathBuf,
    process::{Command, Output},
    str::FromStr,
};

/// Provides the bitstream that can be flashed onto an iCE40-HX1KTQ144 stick
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

    let output = Command::new("yosys")
        .current_dir(dir.clone())
        .arg(r#"-p  synth_ice40 -top top -json top.json"#)
        .arg("top.v")
        .output()?;

    log_out_and_err(output, &dir, "yosys_synth")?;

    let output = Command::new("nextpnr-ice40")
        .current_dir(dir.clone())
        .args([
            "--hx1k",
            "--package",
            "tq144",
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

/// Provides logging of the executed command outputs
fn log_out_and_err(output: Output, dir: &PathBuf, basename: &str) -> Result<(), std::io::Error> {
    let (stdout, stderr) = (
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    );

    let mut out_file = File::create(dir.clone().join(format!("{}.out", basename)))?;
    write!(out_file, "{}", stdout)?;

    let mut err_file = File::create(dir.clone().join(format!("{}.err", basename)))?;
    write!(err_file, "{}", stderr)?;

    Ok(())
}
