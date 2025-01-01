use crate::decimal_matrix_3d::DecimalMatrix3d;
use crate::decimal_vector_3d::DecimalVector3d;
use crate::sin_cos::{cos, f64_to_dbig, sin};
use dashu_float::ops::Abs;
use dashu_float::DBig;
use std::ops::Neg;
use std::str::FromStr;
use std::time::Instant;

mod decimal_matrix_3d;
mod decimal_vector_3d;
mod sin_cos;

fn main() {
    let mut axis = DecimalVector3d::from_f64(1.0, 0.2, 0.3);
    axis.normalize();
    let angle = f64_to_dbig(2.1415);

    let start = Instant::now();
    let matrix = DecimalMatrix3d::axis_angle(axis.clone(), angle);
    let duration = start.elapsed();
    axis = matrix.apply(axis.clone());

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
