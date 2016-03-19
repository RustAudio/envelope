extern crate interpolation as interpolation_lib;
extern crate num;

pub use interpolation_lib as interpolation;

pub use bezier_point::BezierPoint;
pub use ease_point::EasePoint;
pub use envelope::{Envelope, Step};
pub use point::Point;

mod bezier_point;
mod ease_point;
mod envelope;
pub mod interpolate;
mod point;
