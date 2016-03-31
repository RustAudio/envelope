use interpolate::{self, Scalar};
use interpolation::Spatial;

/// Implement this for types to be used as points on an Envelope.
pub trait Point: Clone {
    type X: PartialEq + Clone;
    type Y: PartialEq + Spatial;

    /// Convert X to Y's Scalar.
    fn x_to_scalar(x: Self::X) -> <Self::Y as Spatial>::Scalar;
    /// X (often associated with time).
    fn x(&self) -> Self::X;
    /// Y (often associated with some value).
    fn y(&self) -> Self::Y;
    /// Interpolate between two points and return y for the given x.
    #[inline]
    fn interpolate(x: Self::X, start: &Self, end: &Self) -> Self::Y
        where <Self::Y as Spatial>::Scalar: Scalar,
    {
        interpolate::linear(x, start, end)
    }
}

