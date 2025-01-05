use core::f64;
use std::fmt::Display;

use num::{Complex, Zero};

#[derive(Debug, Default)]
pub struct Layer {
    pub name: String,
    pub n_col: [Complex<f64>; 3],
    pub d: f64,
}

impl Layer {
    pub fn set_name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }
    pub fn set_d(mut self, d: f64) -> Self {
        self.d = d;
        self
    }
    pub fn set_n_col(mut self, cols: [Complex<f64>; 3]) -> Self {
        self.n_col = cols;
        self
    }
    pub fn delta(&self, lambdas: [f64; 3]) -> [Complex<f64>; 3] {
        let mut deltas: [Complex<f64>; 3] = [Complex::zero(); 3];
        for i in 0..3 {
            deltas[i] = 4.0 * f64::consts::PI / lambdas[i] * self.n_col[i] * self.d
        }
        deltas
    }
    pub fn delta_d(&self, lambdas: [f64; 3], d: f64) -> [Complex<f64>; 3] {
        let mut deltas: [Complex<f64>; 3] = [Complex::zero(); 3];
        for i in 0..3 {
            deltas[i] = (4.0 * f64::consts::PI * self.n_col[i] * d) / lambdas[i]
        }
        deltas
    }

    // pub fn delta(&self, lambda: f64, col: usize) -> Complex<f64> {
    //     ((4.0 * f64::consts::PI) / lambda) * self.n_col[col] * self.d
    // }
}

impl Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "*------------------------------------------------*")?;
        writeln!(f, "|  [{:^22}] (d={:12.4} nm)  |", self.name, self.d)?;
        writeln!(
            f,
            "| r={:12.3}  g={:12.3}  b={:12.3} |",
            self.n_col[0], self.n_col[1], self.n_col[2]
        )?;
        write!(f, "*------------------------------------------------*")
    }
}
