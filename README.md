# hdl-eval
Evaluating rust based hardware definition languages:
* [rust-hdl](https://github.com/samitbasu/rust-hdl) by samitbasu
* [rhdl](https://github.com/samitbasu/rhdl/) by samitbasu

Other tools used:
* [yosys](https://github.com/YosysHQ/yosys): open source synthesis suite
* [icestorm](https://github.com/YosysHQ/icestorm): bitstream creation for iCE40 `fpga`s
* [nextpnr](https://github.com/YosysHQ/nextpnr): `fpga` place and route tool
* [surfer](https://gitlab.com/surfer-project/surfer): waveform viewer

## rust-hdl-eval

### How to run
To run the `rust-hdl` evaluation navigate to the `rust-hdl-eval` directory and run `cargo run`. This will default to a test simulation. Running `cargo run -- --help` will provide more information on the existing parameters to run the `rust-hdl` evaluation.

### Modes
The implementation includes 5 seperate modes:
* `Basic`: simple Blinky simulation
    * Output: `vcd` file
* `Testing`: simulation playground regarding the `iCEstick` synthesis
    * Output: `vcd` file
* `Adder`: simulates a generic bit adder including a test bench
    * Output: none; runs the simulation and prints to console
* `Synced` and `Asynced`: flashes a bit stream into the `iCEstick Evaluation Kit`
    * `Synced`: The 5 LEDs on the stick will blink in unison
    * `Asynced`: The 5 LEDs on the stick will blink in a circular motion
    * Output: `.v`, `.pcf`, `json`, `.asc` and the `.bin` files
    * Flashing: The `.bin` file will be flashed onto the `iCEstick`.

## rust-hdl-bsp-ice40-boards 
This library provides a blueprint on how to write board support packages (`BSP`s) for `iCE40` based `fpga` boards.
* A finished implementation for the `iCEstick Evaluation Kit` is included and can be used.
* To include other chips/boards in the provided synthesis, the `pins` module must be implemented for the specific board, and the correct instance of an `Ice40ChipType` struct must be used in the abstracted `synth` methods. 

## rust-hdl prerequisites:
* To enable successful transpilation the open source `icestorm` pipeline, `nextpnr` and `yosys` should be [installed](https://prjicestorm.readthedocs.io/en/latest/overview.html#where-are-the-tools-how-to-install).
* If any OS besides `ubuntu` is used (e.g. `Windows`), additional steps, such as installing drivers for the `iCEstick Evaluation Kit` (or other boards), might be necessary.

## rhdl-eval
* Includes a (probably) correct implementation of a blinker block
* No official documentation for `rhdl` is currently available (2025-08-19) and this makes simulation and flashing onto physical boards a lot harder than it was with `rust-hdl`
* Will try to include this in the future!

## rhdl-bsp-ice-stick
* Includes WIP implementation to support synthesis on the `iCEstick Evaluation Kit`
* Writing drivers (or `BSP`s) for `rhdl` is a lot more involved in comparison to `rust-hdl`.
    * `rust-hdl` utilizes existing open source synthesis tools and targets verilog (via transpilation).
    * `rhdl` on the other hand aims to achieve all of this within native `Rust`.
    * There are currently no clear guidelines (2025-08-19) to the process
