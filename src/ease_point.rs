
use interpolation::{Ease, EaseFunction, Spatial};
use point::Point;
use std::num::{Float, NumCast};


/// A type whose interpolation may involve some sort of easing.
#[derive(Clone, Copy)]
pub struct EasePoint<X, Y>
    where
        X: NumCast + Clone + Copy,
        Y: Spatial + Clone + Copy,
        Y::Scalar: Float + Ease,
{
    pub x: X,
    pub y: Y,
    pub maybe_ease_fn: Option<EaseFunction>,
}


impl<X, Y> EasePoint<X, Y>
    where
        X: NumCast + Clone + Copy,
        Y: Spatial + Clone + Copy,
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


impl<X, Y> Point<X, Y> for EasePoint<X, Y>
    where
        X: NumCast + Clone + Copy,
        Y: Spatial + Clone + Copy,
        Y::Scalar: Float + Ease,
{
    #[inline(always)]
    fn x_to_scalar(&self, x: X) -> Y::Scalar {
        NumCast::from(x).unwrap()
    }
    #[inline(always)]
    fn x(&self) -> X { self.x }
    #[inline(always)]
    fn y(&self) -> Y { self.y }
    #[inline(always)]
    fn interpolate(x: X, start: &EasePoint<X, Y>, end: &EasePoint<X, Y>) -> Y {
        let x: Y::Scalar = start.x_to_scalar(x);
        let start_x: Y::Scalar = start.x_to_scalar(start.x());
        let end_x: Y::Scalar = start.x_to_scalar(end.x());
        let scalar = (x - start_x) / (end_x - start_x);
        let eased_scalar = match start.maybe_ease_fn {
            Some(f) => Ease::calc(scalar, f),
            None => scalar,
        };
        end.y().sub(&start.y()).scale(&eased_scalar)
    }
}
