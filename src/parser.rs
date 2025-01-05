use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::{
        complete::{char, multispace0, newline},
        is_alphanumeric,
    },
    combinator::{map, map_res, opt, rest},
    error::Error,
    multi::{many0, separated_list0},
    number::complete::double,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};
use num::Complex;

use crate::{layer::Layer, structure::Structure};

fn is_id_char(c: char) -> bool {
    c.is_alphanumeric() || c.eq(&'_')
}

fn parse_ident(i: &str) -> IResult<&str, &str> {
    take_while(is_id_char)(i)
}

fn parse_var(i: &str) -> IResult<&str, (&str, f64)> {
    let (i, _) = multispace0(i)?;
    let (i, name) = parse_ident(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = char(':')(i)?;
    let (i, _) = multispace0(i)?;
    let (i, val) = double(i)?;
    Ok((i, (name, val)))
}

fn parse_complex(i: &str) -> IResult<&str, Complex<f64>> {
    let (i, _) = char('(')(i)?;
    let (i, _) = multispace0(i)?;
    let (i, re) = double(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = char(',')(i)?;
    let (i, _) = multispace0(i)?;
    let (i, im) = double(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = char(')')(i)?;
    Ok((i, Complex::new(re, im)))
}

fn parse_n_col(i: &str) -> IResult<&str, [Complex<f64>; 3]> {
    let (i, _) = char('{')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, r) = parse_complex(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = char(',')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, g) = parse_complex(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = char(',')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, b) = parse_complex(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = char('}')(i)?;
    Ok((i, [r, g, b]))
}

fn parse_layer(i: &str) -> IResult<&str, Layer> {
    let mut layer = Layer::default();

    let (i, _) = multispace0(i)?;
    let (i, name) = parse_ident(i)?;
    let (i, _) = multispace0(i)?;

    let (i, _) = char('[')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, n_col) = parse_n_col(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = opt(char(','))(i)?;
    let (i, _) = multispace0(i)?;

    let (i, d) = double(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = char(']')(i)?;

    Ok((i, layer.set_name(name).set_n_col(n_col).set_d(d)))
}

pub fn parse_file(file: PathBuf) -> (HashMap<String, f64>, Structure) {
    let mut file = File::open(file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content);

    let lines = content.lines().collect::<Vec<_>>();

    let mut variables: HashMap<String, f64> = HashMap::new();
    let mut layers = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        if line.trim().starts_with('#') {
            continue;
        }
        if let (i, Some((name, val))) = opt(parse_var)(line).unwrap() {
            variables.insert(name.into(), val);
        }

        if let (i, Some(layer)) = opt(parse_layer)(line).unwrap() {
            layers.push(layer);
        }
    }
    // layers.reverse();
    let rgblam = {
        let red = variables.get("lambda_red").unwrap();
        let green = variables.get("lambda_green").unwrap();
        let blue = variables.get("lambda_blue").unwrap();
        [*red, *green, *blue]
    };
    (variables, Structure::from(layers).set_lambda(rgblam))
}
