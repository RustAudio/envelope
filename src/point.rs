
use interpolation::Spatial;
use std::num::Float;


/// Implement this for types to be used as points on an Envelope.
pub trait Point<X, Y>: Copy + Clone
    where
        Y: Spatial,
        Y::Scalar: Float,
{
    /// Convert X to Y's Scalar.
    fn x_to_scalar(&self, x: X) -> Y::Scalar;
    /// Return the x co-ordinate in scalar form (often associated with time).
    fn x(&self) -> X;
    /// Return the y co-ordinate (often associated with some value).
    fn y(&self) -> Y;
    /// Interpolate between two points and return y for the given x.
    #[inline]
    fn interpolate(x: X, start: &Self, end: &Self) -> Y {
        let x = start.x_to_scalar(x);
        let start_x = start.x_to_scalar(start.x());
        let end_x = start.x_to_scalar(end.x());
        let scalar = (x - start_x) / (end_x - start_x);
        end.y().sub(&start.y()).scale(&scalar)
    }
}

