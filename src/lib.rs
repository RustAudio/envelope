
#![feature(std_misc)]

extern crate "rustc-serialize" as rustc_serialize;

use std::cmp::Ordering;
use std::fmt::Debug;
use std::num::Float;

/// Implement this for types to be used as points on an Envelope.
pub trait Point: Copy + Clone + Debug {
    type F: Float;
    /// Return the x co-ordinate (often associated with time).
    fn x(&self) -> <Self as Point>::F;
    /// Return the y co-ordinate (often associated with some value).
    fn y(&self) -> <Self as Point>::F;
    /// The depth of the bezier curve between this point and the following point.
    fn curve(&self) -> <Self as Point>::F { Float::zero() }
    /// Constructor for a type that implements trait from the curve and the x, y co-ordinates.
    fn new(x: <Self as Point>::F, y: <Self as Point>::F, curve: <Self as Point>::F) -> Self;
}

/// Envelope struct, primarily used for interpolation between a given `Point` type.
#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct Envelope<P> {
    /// Envelope represented by a vector
    /// of points (sorted by `time`).
    pub points: Vec<P>
}

impl<P> Envelope<P> where P: Point, <P as Point>::F: Float {

    /// Construct an empty envelope.
    #[inline]
    pub fn new() -> Envelope<P> {
        Envelope { points: Vec::new() }
    }

    /// Create a new envelope from a Vec<P> and make sure that they're sorted.
    #[inline]
    pub fn from_points(mut points: Vec<P>) -> Envelope<P> {
        points.sort_by(|a, b| if a.x() < b.x() { Ordering::Less }
                              else if a.x() > b.x() { Ordering::Greater }
                              else { Ordering::Equal });
        Envelope { points: points }
    }

    /// Create an envelope zeroed at the start and end.
    #[inline]
    pub fn zeroed() -> Envelope<P> {
        Envelope {
            points: vec![Point::new(Float::zero(), Float::zero(), Float::zero()),
                         Point::new(Float::one(),  Float::zero(), Float::zero())]
        }
    }

    /// Add a new point to the Envelope.
    pub fn add_point(&mut self, point: P) {
        self.points.push(point);
        self.points.sort_by(|a, b| if a.x() < b.x() { Ordering::Less }
                                   else if a.x() > b.x() { Ordering::Greater }
                                   else { Ordering::Equal });
    }

    /// Return `value` for the given `time`.
    ///
    /// If there is less than two points interpolation
    /// is not meaningful, thus we should just return 0.
    #[inline]
    pub fn y(&self, x: <P as Point>::F) -> <P as Point>::F {
        if self.points.len() <= 1 { return Float::zero() }
        let mut i = 1;
        while i < self.points.len() - 1 && x >= self.points[i].x() {
            i += 1;
        }
        interpolate(x, self.points[i-1], self.points[i])
    }

}

/// Interpolate between two points and return y for the given x.
#[inline]
fn interpolate<P>(x: <P as Point>::F, start: P, end: P) -> <P as Point>::F
    where
        P: Point,
        <P as Point>::F: Float,
{
    // Find x passed from start of interpolation.
    let x_pos = x - start.x();

    // Find duration of interpolation.
    let duration = end.x() - start.x();

    // Set gradient for interpolation.
    let gradient_y = end.y() - start.y();

    // If there is no gradient between the points, simply return y from one of the points.
    if gradient_y == Float::zero() { return start.y() }

    let half_gradient_y: <P as Point>::F = gradient_y / two();

    // Consider bezier curve.
    let y2 = half_gradient_y + start.curve() * half_gradient_y;
    let perc_x = x_pos / duration;

    // Re-adjust linear trajectory.
    let ya = bezier_pt(Float::zero(), y2, perc_x);
    let yb = bezier_pt(y2, gradient_y, perc_x);

    bezier_pt(ya, yb, perc_x) + start.y()
}

/// Get bezier point for bezier curve.
#[inline]
fn bezier_pt<F>(n1: F, n2: F, perc: F) -> F where F: Float {
    (n2 - n1) * perc + n1
}

/// Return 2.0.
#[inline]
fn two<F>() -> F where F: Float {
    let one: F = Float::one();
    one + one
}


