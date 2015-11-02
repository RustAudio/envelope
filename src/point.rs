
use interpolate;
use interpolation::Spatial;
use num::Float;


/// Implement this for types to be used as points on an Envelope.
pub trait Point<X, Y>: Copy + Clone {
    /// Convert X to Y's Scalar.
    fn x_to_scalar(x: X) -> Y::Scalar where Y: Spatial;
    /// X (often associated with time).
    fn x(&self) -> X;
    /// Y (often associated with some value).
    fn y(&self) -> Y;
    /// Interpolate between two points and return y for the given x.
    #[inline]
    fn interpolate(x: X, start: &Self, end: &Self) -> Y where
        X: PartialEq,
        Y: PartialEq + Spatial,
        Y::Scalar: Float,
    {
        interpolate::linear(x, start, end)
    }
}

