
extern crate interpolation;
extern crate num;
extern crate rustc_serialize;

pub use bezier_point::BezierPoint;
pub use ease_point::EasePoint;
pub use envelope::Envelope;
pub use point::Point;

mod bezier_point;
mod ease_point;
mod envelope;
mod point;

