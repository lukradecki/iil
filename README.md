# Application generating data for maps
---
## Building 
1. setup rust enviroment: [Rust install](https://www.rust-lang.org/tools/install)
2. clone repo: git clone https://github.com/lukradecki/iil
3. cd into repo
4. `cargo build --release` to build package or `cargo run` to run it
   
#### *after running `cargo build --release`, binary is in `target/release`*
---
## Defining material structure
basic example `layers.str` contains example structure with comments describing elements.
---
## Basic command to generate data from existing file
`<binary_name> -i <material_structure> -o data.data`
