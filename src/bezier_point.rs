
use interpolation::Spatial;
use num::{Float, NumCast};
use std::fmt::Debug;
use point::Point;


/// A type whose interpolation may involve some quadratic bezier curve.
#[derive(Debug, Clone, Copy, RustcEncodable, RustcDecodable)]
pub struct BezierPoint<X, Y>
    where
        X: NumCast + Clone + Copy + Debug,
        Y: Float + Debug,
{
    pub x: X,
    pub y: Y,
    pub curve: Y,
}


impl<X, Y> BezierPoint<X, Y>
    where
        X: NumCast + Clone + Copy + Debug,
        Y: Float + Debug,
{
    /// Constructor for a BezierPoint.
    #[inline]
    pub fn new(x: X, y: Y, curve: Y) -> BezierPoint<X, Y> {
        BezierPoint {
            x: x,
            y: y,
            curve: curve,
        }
    }
}


impl<X, Y> Point<X, Y> for BezierPoint<X, Y>
    where
        X: NumCast + Clone + Copy + Debug,
        Y: Spatial + Float + Debug,
        Y::Scalar: Float,
{
    #[inline(always)]
    fn x_to_scalar(x: X) -> Y::Scalar {
        NumCast::from(x).unwrap()
    }
    #[inline(always)]
    fn x(&self) -> X { self.x }
    #[inline(always)]
    fn y(&self) -> Y { self.y }
    /// Interpolate between two points and return y for the given x.
    #[inline(always)]
    fn interpolate(x: X, start: &BezierPoint<X, Y>, end: &BezierPoint<X, Y>) -> Y {
        // If there is no gradient between the points, simply return y from one of the points.
        if start.y == end.y { return start.y }
        let x: Y = NumCast::from(x).unwrap();
        let start_x: Y = NumCast::from(start.x()).unwrap();
        let end_x: Y = NumCast::from(end.x()).unwrap();
        // Find x passed from start of interpolation.
        let x_pos = x - start_x;
        // Find duration of interpolation.
        let duration = end_x - start_x;
        // Set gradient for interpolation.
        let gradient_y = end.y - start.y;
        let half_gradient_y: Y = gradient_y / two();
        // Consider bezier curve.
        let y2 = half_gradient_y + start.curve * half_gradient_y;
        let perc_x = x_pos / duration;
        // Re-adjust linear trajectory.
        let ya = bezier_pt(Y::zero(), y2, perc_x);
        let yb = bezier_pt(y2, gradient_y, perc_x);
        bezier_pt(ya, yb, perc_x) + start.y
    }
}


/// Get bezier point for bezier curve.
#[inline]
fn bezier_pt<A>(n1: A, n2: A, perc: A) -> A
    where
        A: Float
{
    (n2 - n1) * perc + n1
}


/// Return 2.0.
#[inline]
fn two<F>() -> F
    where
        F: Float
{
    let one: F = F::one();
    one + one
}

