
use interpolation::{Ease, EaseFunction, Spatial};
use num::{Float, NumCast};
use point::Point;


/// Check if the given x lands exactly on either the start or end point and return the Y value if
/// it does.
#[inline]
fn maybe_exact_point<X, Y, P>(x: &X, start: &P, end: &P) -> Option<Y> where
    P: Point<X, Y>,
    X: PartialEq,
    Y: Spatial + PartialEq,
    Y::Scalar: Float,
{
    // No need to interpolate if:
    // - both y values are the same
    // - start_x and x are the same
    // - end_x and x are the same
    if start.y() == end.y() || start.x() == *x {
        Some(start.y())
    } else if end.x() == *x {
        Some(end.y())
    } else {
        None
    }
}


/// Interpolate linearly between the start and end points.
#[inline]
pub fn linear<X, Y, P>(x: X, start: &P, end: &P) -> Y where
    P: Point<X, Y>,
    X: PartialEq,
    Y: Spatial + PartialEq,
    Y::Scalar: Float,
{
    maybe_exact_point(&x, start, end).unwrap_or_else(|| {
        let x = P::x_to_scalar(x);
        let start_x = P::x_to_scalar(start.x());
        let end_x = P::x_to_scalar(end.x());
        let scalar = (x - start_x) / (end_x - start_x);
        let difference = end.y().sub(&start.y());
        let interpolated_difference = difference.scale(&scalar);
        start.y().add(&interpolated_difference)
    })
}


/// Interpolate between the start and end points using the given easing function.
#[inline]
pub fn ease<X, Y, P>(x: X, start: &P, end: &P, ease_fn: EaseFunction) -> Y where
    P: Point<X, Y>,
    X: PartialEq,
    Y: Spatial + PartialEq,
    Y::Scalar: Float + Ease,
{
    maybe_exact_point(&x, start, end).unwrap_or_else(|| {
        let x = P::x_to_scalar(x);
        let start_x = P::x_to_scalar(start.x());
        let end_x = P::x_to_scalar(end.x());
        let scalar = (x - start_x) / (end_x - start_x);
        let eased_scalar = Ease::calc(scalar, ease_fn);
        let difference = end.y().sub(&start.y());
        let interpolated_difference = difference.scale(&eased_scalar);
        start.y().add(&interpolated_difference)
    })
}


/// Interpolate between the given start and end points given some bezier curve.
#[inline]
pub fn bezier<X, Y, P>(x: X, start: &P, end: &P, curve: Y::Scalar) -> Y where
    P: Point<X, Y>,
    X: PartialEq,
    Y: Spatial + NumCast + PartialEq,
    Y::Scalar: Float,
{
    maybe_exact_point(&x, start, end).unwrap_or_else(|| {

        /// Get bezier point for bezier curve.
        #[inline]
        fn bezier_pt<A>(n1: A, n2: A, perc: A) -> A where A: Float {
            (n2 - n1) * perc + n1
        }

        /// Return 2.0.
        #[inline]
        fn two<F>() -> F where F: Float {
            let one: F = F::one();
            one + one
        }

        let x = P::x_to_scalar(x);
        let start_x = P::x_to_scalar(start.x());
        let end_x = P::x_to_scalar(end.x());
        // Find x passed from start of interpolation.
        let x_pos = x - start_x;
        // Find duration of interpolation.
        let duration = end_x - start_x;
        // Set gradient for interpolation.
        let end_y: Y::Scalar = NumCast::from(end.y()).unwrap();
        let start_y: Y::Scalar = NumCast::from(start.y()).unwrap();
        let gradient_y: Y::Scalar = end_y - start_y;
        let half_gradient_y: Y::Scalar = gradient_y / two();
        // Consider bezier curve.
        let y2 = half_gradient_y + curve * half_gradient_y;
        let perc_x = x_pos / duration;
        // Re-adjust linear trajectory.
        let zero: Y::Scalar = NumCast::from(0.0).unwrap();
        let ya = bezier_pt(zero, y2, perc_x);
        let yb = bezier_pt(y2, gradient_y, perc_x);
        let y = NumCast::from(bezier_pt(ya, yb, perc_x)).unwrap();
        start.y().add(&y)
    })
}

