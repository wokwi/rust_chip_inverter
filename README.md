# Wokwi Inverter Chip in Rust example

## Building

To build:

```
cargo build --target wasm32-unknown-unknown --release
```

Find the resulting binary in `target/wasm32-unknown-unknown/release/chip_inverter.wasm`

## Simulating

To simulate this project, install [Wokwi for VS Code](https://marketplace.visualstudio.com/items?itemName=wokwi.wokwi-vscode). Open the project directory in Visual Studio Code, press **F1** and select "Wokwi: Start Simulator".
