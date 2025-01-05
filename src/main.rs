#![allow(dead_code, unused)]
mod layer;
mod parser;
mod structure;

use core::f64;
use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufWriter, Read, Write},
    path::PathBuf,
};

use argh::FromArgs;
use num::Complex;
use parser::parse_file;
/// Cli
#[derive(FromArgs)]
pub struct Cli {
    /// print material structure
    #[argh(switch, short = 's')]
    show: bool,

    /// where to save generated data
    #[argh(option, short = 'o')]
    output: PathBuf,

    /// file with defined consts and struct eg. layers.str
    #[argh(option, short = 'i')]
    input: PathBuf,

    /// overwrite depth from file
    #[argh(option, short = 'd')]
    depth: Option<f64>,

    /// overwrite point density from file
    #[argh(option, short = 'p')]
    point: Option<usize>,
}

fn main() {
    let cli: Cli = argh::from_env();
    let (vars, mut structure) = parse_file(cli.input);
    if cli.show {
        println!("{structure}");
    }
    let point_density = cli.point.unwrap_or(
        *vars
            .get("point_density")
            .expect("you need to define 'point_density: <value>'") as usize,
    );
    let depth = cli.depth.unwrap_or(
        *vars
            .get("depth")
            .expect("you need to define 'depth: <value>'"),
    );

    let mut file = File::create(cli.output).unwrap();
    let mut bufwriter = BufWriter::new(file);

    for n in ndarray::linspace(0.0, depth, point_density) {
        let val = structure.intensity(n);
        writeln!(bufwriter, "{}\t{}\t{}\t{}", n, val[0], val[1], val[2]);
    }
}
