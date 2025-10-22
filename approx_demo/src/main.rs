#[macro_use]
extern crate approx;

use std::f64;
fn main() {
    abs_diff_eq!(1.0, 1.0);
    abs_diff_eq!(1.0, 1.0, epsilon = f64::EPSILON);

    relative_eq!(1.0, 1.0);
    relative_eq!(1.0, 1.0, epsilon = f64::EPSILON);
    relative_eq!(1.0, 1.0, max_relative = 1.0);
    relative_eq!(1.0, 1.0, epsilon = f64::EPSILON, max_relative = 1.0);
    relative_eq!(1.0, 1.0, max_relative = 1.0, epsilon = f64::EPSILON);

    ulps_eq!(1.0, 1.0);
    ulps_eq!(1.0, 1.0, epsilon = f64::EPSILON);
    ulps_eq!(1.0, 1.0, max_ulps = 4);
    ulps_eq!(1.0, 1.0, epsilon = f64::EPSILON, max_ulps = 4);
    ulps_eq!(1.0, 1.0, max_ulps = 4, epsilon = f64::EPSILON);
}
