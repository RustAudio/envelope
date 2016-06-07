pub extern crate interpolation;
extern crate num;

pub use bezier_point::BezierPoint;
pub use ease_point::EasePoint;
pub use envelope::{Envelope, Steps};
pub use point::Point;

mod bezier_point;
mod ease_point;
mod envelope;
pub mod interpolate;
mod point;

#[cfg(feature="serde_serialization")]
mod serde;
