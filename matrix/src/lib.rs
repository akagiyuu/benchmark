#![feature(test)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

mod flat_matrix;
mod matrix;
mod number;

pub use matrix::Matrix;
pub use flat_matrix::FlatMatrix;
