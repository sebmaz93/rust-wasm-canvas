# rust-wasm-canvas

This guide provides step-by-step instructions for setting up a Rust project to compile to WebAssembly (Wasm) and run it in an HTML file.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

## Installation

### Step 1: Install Rust

1. Open your terminal.
2. Run the following command to install Rust:

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

3. Install wasm-pack
   ```sh
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

4. Build the project
   ```sh
   wasm-pack build --target web
   
5. Serve the HTML file which includes the imported Script
for ex: you can use python
   ```sh
   python3 -m http.server