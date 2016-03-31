use interpolate;
use interpolation::{Ease, EaseFunction, Spatial};
use point::Point;
use num::{Float, NumCast};


/// A type whose interpolation may involve some sort of easing.
#[derive(Clone, Copy)]
pub struct EasePoint<X, Y>
    where X: PartialEq + NumCast + Clone + Copy,
          Y: PartialEq + Spatial + Clone + Copy,
          Y::Scalar: Float + Ease,
{
    pub x: X,
    pub y: Y,
    pub maybe_ease_fn: Option<EaseFunction>,
}


impl<X, Y> EasePoint<X, Y>
    where X: PartialEq + NumCast + Clone + Copy,
          Y: PartialEq + Spatial + Clone + Copy,
          Y::Scalar: Float + Ease,
{
    /// Constructor for an EasePoint.
    #[inline]
    pub fn new(x: X, y: Y, maybe_ease_fn: Option<EaseFunction>) -> EasePoint<X, Y> {
        EasePoint {
            x: x,
            y: y,
            maybe_ease_fn: maybe_ease_fn,
        }
    }
}


impl<X, Y> Point for EasePoint<X, Y>
    where X: PartialEq + NumCast + Clone + Copy,
          Y: PartialEq + Spatial + Clone + Copy,
          Y::Scalar: Float + Ease,
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
    #[inline(always)]
    fn interpolate(x: X, start: &Self, end: &Self) -> Y {
        match start.maybe_ease_fn {
            Some(ease_fn) => interpolate::ease(x, start, end, ease_fn),
            None => interpolate::linear(x, start, end),
        }
    }
}

