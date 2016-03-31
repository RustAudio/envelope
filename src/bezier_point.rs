use interpolate;
use interpolation::Spatial;
use num::{Float, NumCast};
use point::Point;


/// A type whose interpolation may involve some quadratic bezier curve.
#[derive(Debug, Clone, Copy)]
pub struct BezierPoint<X, Y> where
    X: Clone + Copy,
    Y: Spatial + Clone + Copy,
    Y::Scalar: Float,
{
    pub x: X,
    pub y: Y,
    pub curve: Y::Scalar,
}


impl<X, Y> BezierPoint<X, Y> where
    X: Clone + Copy,
    Y: Spatial + Clone + Copy,
    Y::Scalar: Float,
{
    /// Constructor for a BezierPoint.
    #[inline]
    pub fn new(x: X, y: Y, curve: Y::Scalar) -> BezierPoint<X, Y> {
        BezierPoint {
            x: x,
            y: y,
            curve: curve,
        }
    }
}


impl<X, Y> Point for BezierPoint<X, Y>
    where X: PartialEq + NumCast + Clone + Copy,
          Y: PartialEq + NumCast + Spatial + Clone + Copy,
          Y::Scalar: Float,
{
    type Scalar = <Y as Spatial>::Scalar;
    type X = X;
    type Y = Y;
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
    fn interpolate(x: X, start: &Self, end: &Self) -> Y where
        X: PartialEq,
        Y: PartialEq,
    {
        interpolate::bezier(x, start, end, start.curve)
    }
}

