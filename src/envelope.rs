
use interpolation::Spatial;
use num::Float;
use point::Point;
use std::cmp::Ordering;


/// For designing a series of interpolation points.
#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct Envelope<P> {
    /// Envelope represented by a vector
    /// of points (sorted by `time`).
    pub points: Vec<P>,
}


impl<P> Envelope<P>
{

    /// Construct an empty envelope.
    #[inline]
    pub fn new() -> Envelope<P> {
        Envelope { points: Vec::new(), }
    }

    /// Create a new envelope from a Vec<P> and make sure that they're sorted.
    #[inline]
    pub fn from_points<X, Y>(mut points: Vec<P>) -> Envelope<P>
        where
            X: PartialOrd,
            Y: Spatial,
            Y::Scalar: Float,
            P: Point<X, Y>,
    {
        points.sort_by(|a, b| if a.x() < b.x() { Ordering::Less }
                              else if a.x() > b.x() { Ordering::Greater }
                              else { Ordering::Equal });
        Envelope {
            points: points,
        }
    }

    /// Add a new point to the Envelope.
    pub fn add_point<X, Y>(&mut self, point: P)
        where
            X: PartialOrd,
            Y: Spatial,
            Y::Scalar: Float,
            P: Point<X, Y>,
    {
        self.points.push(point);
        self.points.sort_by(|a, b| if a.x() < b.x() { Ordering::Less }
                                   else if a.x() > b.x() { Ordering::Greater }
                                   else { Ordering::Equal });
    }

    /// Return `y` for the given `x`.
    ///
    /// If there is less than two points interpolation
    /// is not meaningful, thus we should just return 0.
    #[inline]
    pub fn y<X, Y>(&self, x: X) -> Option<Y>
        where
            X: PartialOrd,
            Y: Spatial + PartialEq,
            Y::Scalar: Float,
            P: Point<X, Y>,
    {
        if self.points.len() <= 1 { return None }
        let mut i = 1;
        while i < self.points.len() - 1 && x >= self.points[i].x() {
            i += 1;
        }
        Some(Point::interpolate(x, &self.points[i-1], &self.points[i]))
    }

}

