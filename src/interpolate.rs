
use interpolation::{Ease, EaseFunction, Spatial};
use num::NumCast;
use point::Point;
use std;


/// Check if the given x lands exactly on either the start or end point and return the Y value if
/// it does.
#[inline]
fn maybe_exact_point<P>(x: &P::X, start: &P, end: &P) -> Option<P::Y>
    where P: Point,
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


/// Set of traits required by <Y as Spatial>::Scalar.
pub trait Scalar: Sized + Clone
    + std::ops::Add<Output=Self>
    + std::ops::Sub<Output=Self>
    + std::ops::Mul<Output=Self>
    + std::ops::Div<Output=Self> {}
impl<T> Scalar for T
    where T: Sized + Clone
        + std::ops::Add<Output=T>
        + std::ops::Sub<Output=T>
        + std::ops::Mul<Output=T>
        + std::ops::Div<Output=T> {}


/// Interpolate linearly between the start and end points.
#[inline]
pub fn linear<P>(x: P::X, start: &P, end: &P) -> P::Y
    where P: Point,
          P::X: Clone,
          <P::Y as Spatial>::Scalar: Scalar,
{
    maybe_exact_point(&x, start, end).unwrap_or_else(|| {
        let x = P::x_to_scalar(x.clone());
        let start_x = P::x_to_scalar(start.x());
        let end_x = P::x_to_scalar(end.x());
        let scalar = (x - start_x.clone()) / (end_x - start_x);
        let difference = end.y().sub(&start.y());
        let interpolated_difference = difference.scale(&scalar);
        start.y().add(&interpolated_difference)
    })
}


/// Interpolate between the start and end points using the given easing function.
#[inline]
pub fn ease<P>(x: P::X, start: &P, end: &P, ease_fn: EaseFunction) -> P::Y
    where P: Point,
          <P::Y as Spatial>::Scalar: Ease + Scalar,
{
    maybe_exact_point(&x, start, end).unwrap_or_else(|| {
        let x = P::x_to_scalar(x);
        let start_x = P::x_to_scalar(start.x());
        let end_x = P::x_to_scalar(end.x());
        let scalar = (x.clone() - start_x.clone()) / (end_x - start_x);
        let eased_scalar = Ease::calc(scalar, ease_fn);
        let difference = end.y().sub(&start.y());
        let interpolated_difference = difference.scale(&eased_scalar);
        start.y().add(&interpolated_difference)
    })
}


/// Interpolate between the given start and end points given some bezier curve.
#[inline]
pub fn bezier<P>(x: P::X, start: &P, end: &P, curve: <P::Y as Spatial>::Scalar) -> P::Y
    where P: Point,
          P::Y: NumCast,
          <P::Y as Spatial>::Scalar: Scalar + NumCast,
{
    maybe_exact_point(&x, start, end).unwrap_or_else(|| {

        /// Get bezier point for bezier curve.
        #[inline]
        fn bezier_pt<T>(n1: T, n2: T, perc: T) -> T
            where T: Scalar
        {
            (n2 - n1.clone()) * perc + n1
        }

        let x = P::x_to_scalar(x.clone());
        let start_x = P::x_to_scalar(start.x());
        let end_x = P::x_to_scalar(end.x());
        // Find x passed from start of interpolation.
        let x_pos = x - start_x.clone();
        // Find duration of interpolation.
        let duration = end_x - start_x;

        // Set gradient for interpolation.
        let end_y: <P::Y as Spatial>::Scalar = NumCast::from(end.y()).unwrap();
        let start_y: <P::Y as Spatial>::Scalar = NumCast::from(start.y()).unwrap();
        let diff_y: <P::Y as Spatial>::Scalar = end_y - start_y;
        let half_diff_y: <P::Y as Spatial>::Scalar = diff_y.clone() / NumCast::from(2.0).unwrap();
        // Consider bezier curve.
        let y2 = half_diff_y.clone() + curve * half_diff_y;
        let perc_x = x_pos / duration;
        // Re-adjust linear trajectory.
        let zero: <P::Y as Spatial>::Scalar = NumCast::from(0.0).unwrap();
        let ya = bezier_pt(zero, y2.clone(), perc_x.clone());
        let yb = bezier_pt(y2, diff_y, perc_x.clone());
        let y = NumCast::from(bezier_pt(ya, yb, perc_x)).unwrap();
        start.y().add(&y)
    })
}
