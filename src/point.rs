
use interpolation::Spatial;
use num::Float;


/// Implement this for types to be used as points on an Envelope.
pub trait Point<X, Y>: Copy + Clone where
    Y: Spatial,
    Y::Scalar: Float,
{
    /// Convert X to Y's Scalar.
    fn x_to_scalar(x: X) -> Y::Scalar;
    /// X (often associated with time).
    fn x(&self) -> X;
    /// Y (often associated with some value).
    fn y(&self) -> Y;
    /// Interpolate between two points and return y for the given x.
    #[inline]
    fn interpolate(x: X, start: &Self, end: &Self) -> Y where
        Y: PartialEq,
        X: PartialEq,
    {
        // No need to interpolate if:
        // - both y values are the same
        // - start_x and x are the same
        // - end_x and x are the same
        if start.y() == end.y() || start.x() == x {
            return start.y();
        } else if end.x() == x {
            return end.y();
        }

        let x = Self::x_to_scalar(x);
        let start_x = Self::x_to_scalar(start.x());
        let end_x = Self::x_to_scalar(end.x());
        let scalar = (x - start_x) / (end_x - start_x);
        end.y().sub(&start.y()).scale(&scalar)
    }
}

