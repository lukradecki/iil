use core::f64;
use std::fmt::Display;

use num::{complex::ComplexFloat, Complex, Zero};

use crate::layer::Layer;

#[derive(Debug)]
pub struct Structure {
    layers: Vec<Layer>,
    lambda: [f64; 3],
    point_density: usize,
}
impl Display for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in self.layers.iter() {
            writeln!(f, "{}", l)?;
        }
        Ok(())
    }
}
impl From<Vec<Layer>> for Structure {
    fn from(value: Vec<Layer>) -> Self {
        Structure {
            layers: value,
            lambda: [0.0; 3],
            point_density: 0,
        }
    }
}

impl Structure {
    pub fn new() -> Self {
        Structure {
            layers: Vec::new(),
            lambda: [0.0; 3],
            point_density: 0,
        }
    }

    pub fn set_lambda(mut self, lambda: [f64; 3]) -> Self {
        self.lambda = lambda;
        self
    }

    fn refrac(&self, k1: usize, k2: usize) -> [Complex<f64>; 3] {
        let mut ret: [Complex<f64>; 3] = [Complex::zero(); 3];
        for i in 0..3 {
            ret[i] = (self.layers[k2].n_col[i] - self.layers[k1].n_col[i])
                / (self.layers[k2].n_col[i] + self.layers[k1].n_col[i])
        }
        ret
    }

    pub fn intensity(&self, depth: f64) -> [f64; 3] {
        let mut ret = [0.0; 3];
        let reie = self.reie(depth);
        for i in 0..3 {
            ret[i] = reie[i].norm().powi(2);
        }
        ret
    }

    //
    pub fn reie(&self, depth: f64) -> [Complex<f64>; 3] {
        let mut depth = depth;
        let max_depth: f64 = self.layers.iter().map(|v| v.d).sum();
        if depth > max_depth {
            println!(
                "[warning] depth ({}) > material depth ({})",
                depth, max_depth
            );
            println!("          setting depth = material depth");
            depth = max_depth;
        }

        let mut depths_per_layer = Vec::new();
        let mut depth_layer_index = 0;

        let mut neg_depth_accum = depth;
        for (i, layer) in self.layers.iter().enumerate() {
            depth_layer_index = i;
            let diff = neg_depth_accum - layer.d;
            if diff <= 0.0 {
                depths_per_layer.push(layer.d + diff);
                break;
            }

            neg_depth_accum -= layer.d;
            depths_per_layer.push(layer.d);
        }
        let sl = &self.layers[0..=depth_layer_index]
            .iter()
            .zip(&depths_per_layer)
            // .zip(&refracs)
            .collect::<Vec<_>>();

        // println!("{sl:#?}");
        //
        // Layer 0
        // --------reie1 = r01
        // Layer 1
        // --------reie2
        // Layer 2
        // --------reie2
        // Layer 3
        // --------reie3
        // Layer 4

        let mut r_iter = 0;
        let mut reie = self.refrac(r_iter, r_iter + 1);
        r_iter += 1;
        let mut rnext = self.refrac(r_iter, r_iter + 1);
        r_iter += 1;
        let mut delta = [Complex::zero(); 3];
        for (l, d) in sl {
            delta = l.delta_d(self.lambda, **d);
            for i in 0..3 {
                let t = (rnext[0] + reie[i] * (-1.0 * Complex::<f64>::I * delta[i]).exp());
                let b = (1.0 + rnext[i] * reie[i] * (-1.0 * Complex::<f64>::I * delta[i]).exp());
                reie[i] = t / b;
            }
            if r_iter < depth_layer_index {
                rnext = self.refrac(r_iter, r_iter + 1);
                r_iter += 1;
            }
        }
        reie
    }
}
