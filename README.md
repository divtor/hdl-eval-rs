# hdl-eval
Evaluating rust based hardware definition languages:
* [rust-hdl](https://github.com/samitbasu/rust-hdl) by samitbasu
* [rhdl](https://github.com/samitbasu/rhdl/) by samitbasu

Other tools used:
* [yosys](https://github.com/YosysHQ/yosys): Open synthesis suite
* [surfer](https://gitlab.com/surfer-project/surfer): Waveform viewer

## Synthesis
Actual synthesis onto a physical `FPGA` uses the Lattice `iCEstick`. The concrete `iCEstick` boards the `iCE40-HX1KTQ144` chip. This project assumes this specific `FPGA`, but can be forked to include other `FPGAs` as well. The necessary tools for synthesis can be found in the quick setup tutorial of the `rust-hdl` documentation. If any OS besides `ubuntu` is used (e.g. `Windows`), additional steps, such as installing drivers for the `iCEstick`, might be necessary.

## rust-hdl
The implementation includes 5 seperate modes:
* `Basic`: simple Blinky simulation
    * Output: `vcd` file
* `Testing`: simulation playground regarding the `iCEstick` synthesis
    * Output: `vcd` file
* `Adder`: simulates a generic bit adder including a test bench
    * Output: none; runs the simulation and prints to console
* `Synced` and `Asynced`: flashes a bit stream into the `iCEstick`
    * `Synced`: The 5 LEDs on the stick will blink in unison
    * `Asynced`: The 5 LEDs on the stick will blink in a circular motion
    * Output: `.v`, `.pcf`, `json`, `.asc` and the `.bin` files
    * Flashing: The `.bin` file will be flashed onto the `iCEstick`.

## rhdl
