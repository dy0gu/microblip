# Micro*blip* 🔴️🔊️

I didn't have any creative ideas for the name this time, sorry.

See the [docs](./docs) folder for datasheets, specifications and schematic references for every component. The project relies on HAL crates so understanding these files is not strictly necessary.

Note that the [.vscode/settings.json](./.vscode/settings.json) file should not be in `gitignore`, otherwise `rust-analyzer` (by default) will constantly attempt to compile the project for the host
architecture, raising false errors. Other IDEs may also require similar configuration.

## Development 🛠️

1. Install the embedded toolkit via cargo: `cargo install probe-rs-tools --locked`

    - Linux users **must** also follow the [UDEV rules configuration](https://probe.rs/docs/getting-started/probe-setup/) step.

2. Connect the microcontroller via USB.

3. Run `cargo embed` to flash the firmware to the micro:bit and initiate the serial connection for
debugging.

    - If the toolkit fails to find a probe, ensure your OS does not have any additional requirements mentioned in the [probe setup](https://probe.rs/docs/getting-started/probe-setup/) docs.

## Release 🚀

- Run `cargo build --release` and check the `target` folder for the compiled output.
