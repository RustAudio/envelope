
use interpolation::Spatial;
use point::Point;
use std::cmp::Ordering;
use std::num::Float;


/// For designing a series of interpolation points.
#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct Envelope<X, Y, P> {
    /// Envelope represented by a vector
    /// of points (sorted by `time`).
    pub points: Vec<P>,
    phantom_x: ::std::marker::PhantomData<X>,
    phantom_y: ::std::marker::PhantomData<Y>,
}


impl<X, Y, P> Envelope<X, Y, P>
    where
        X: PartialOrd,
        Y: Spatial,
        Y::Scalar: Float,
        P: Point<X, Y>,
{

    /// Construct an empty envelope.
    #[inline]
    pub fn new() -> Envelope<X, Y, P> {
        Envelope {
            points: Vec::new(),
            phantom_x: ::std::marker::PhantomData,
            phantom_y: ::std::marker::PhantomData,
        }
    }

    /// Create a new envelope from a Vec<P> and make sure that they're sorted.
    #[inline]
    pub fn from_points(mut points: Vec<P>) -> Envelope<X, Y, P> {
        points.sort_by(|a, b| if a.x() < b.x() { Ordering::Less }
                              else if a.x() > b.x() { Ordering::Greater }
                              else { Ordering::Equal });
        Envelope {
            points: points,
            phantom_x: ::std::marker::PhantomData,
            phantom_y: ::std::marker::PhantomData,
        }
    }

    /// Add a new point to the Envelope.
    pub fn add_point(&mut self, point: P) {
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
    pub fn y(&self, x: X) -> Option<Y> {
        if self.points.len() <= 1 { return None }
        let mut i = 1;
        while i < self.points.len() - 1 && x >= self.points[i].x() {
            i += 1;
        }
        Some(Point::interpolate(x, &self.points[i-1], &self.points[i]))
    }

}

