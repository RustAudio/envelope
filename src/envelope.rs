
use interpolation::Spatial;
use num::Float;
use point::Point;
use std::iter::IntoIterator;


/// Types representable as an Envelope.
pub trait Envelope<P> {

    /// Construct an `Envelope` from an iterator producing `Point`s.
    fn from_points<I: IntoIterator<Item=P>>(points: I) -> Self;

    /// A reference to the `Envelope`'s slice of `Point`s.
    fn points(&self) -> &[P];

    /// The index of the `Point` that comes directly before the given `x`.
    #[inline]
    fn point_idx_before<X, Y>(&self, x: X) -> Option<usize> where
        P: Point<X, Y>,
        X: PartialOrd,
        Y: Spatial,
        Y::Scalar: Float,
    {
        self.points().iter().enumerate().take_while(|&(_, point)| point.x() < x).last()
            .map(|(i, _)| i)
    }

    /// The index of the `Point` that either lands on or comes directly before the given `x`.
    #[inline]
    fn point_idx_on_or_before<X, Y>(&self, x: X) -> Option<usize> where
        P: Point<X, Y>,
        X: PartialOrd,
        Y: Spatial,
        Y::Scalar: Float,
    {
        self.points().iter().enumerate().take_while(|&(_, point)| point.x() <= x).last()
            .map(|(i, _)| i)
    }

    /// The index of the `Point` that comes directly after the given `x`.
    #[inline]
    fn point_idx_after<X, Y>(&self, x: X) -> Option<usize> where
        P: Point<X, Y>,
        X: PartialOrd,
        Y: Spatial,
        Y::Scalar: Float,
    {
        self.points().iter().enumerate().rev().take_while(|&(_, point)| point.x() > x).last()
            .map(|(i, _)| i)
    }

    /// The index of the `Point` that comes directly after the given `x`.
    #[inline]
    fn point_idx_on_or_after<X, Y>(&self, x: X) -> Option<usize> where
        P: Point<X, Y>,
        X: PartialOrd,
        Y: Spatial,
        Y::Scalar: Float,
    {
        self.points().iter().enumerate().rev().take_while(|&(_, point)| point.x() >= x).last()
            .map(|(i, _)| i)
    }

    /// A reference to the first point that comes before the given `x`.
    #[inline]
    fn point_before<X, Y>(&self, x: X) -> Option<&P> where
        P: Point<X, Y>,
        X: PartialOrd,
        Y: Spatial,
        Y::Scalar: Float,
    {
        self.point_idx_before(x).map(|i| &self.points()[i])
    }

    /// A reference to the first point that is equal to or comes before the given `x`.
    #[inline]
    fn point_on_or_before<X, Y>(&self, x: X) -> Option<&P> where
        P: Point<X, Y>,
        X: PartialOrd,
        Y: Spatial,
        Y::Scalar: Float,
    {
        self.point_idx_on_or_before(x).map(|i| &self.points()[i])
    }

    /// A reference to the first point that comes before the given `x` along with its index.
    #[inline]
    fn point_before_with_idx<X, Y>(&self, x: X) -> Option<(usize, &P)> where
        P: Point<X, Y>,
        X: PartialOrd,
        Y: Spatial,
        Y::Scalar: Float,
    {
        self.point_idx_before(x).map(|i| (i, &self.points()[i]))
    }

    /// A reference to the first point that is equal to or comes before the given `x` along with
    /// its index.
    #[inline]
    fn point_on_or_before_with_idx<X, Y>(&self, x: X) -> Option<(usize, &P)> where
        P: Point<X, Y>,
        X: PartialOrd,
        Y: Spatial,
        Y::Scalar: Float,
    {
        self.point_idx_on_or_before(x).map(|i| (i, &self.points()[i]))
    }

    /// A reference to the first point that comes after the given `x`.
    #[inline]
    fn point_after<X, Y>(&self, x: X) -> Option<&P> where
        P: Point<X, Y>,
        X: PartialOrd,
        Y: Spatial,
        Y::Scalar: Float,
    {
        self.point_idx_after(x).map(|i| &self.points()[i])
    }

    /// A reference to the first point that is equal to or comes after the given `x`.
    #[inline]
    fn point_on_or_after<X, Y>(&self, x: X) -> Option<&P> where
        P: Point<X, Y>,
        X: PartialOrd,
        Y: Spatial,
        Y::Scalar: Float,
    {
        self.point_idx_on_or_after(x).map(|i| &self.points()[i])
    }

    /// A reference to the first point that comes after the given `x` along with its index.
    #[inline]
    fn point_after_with_idx<X, Y>(&self, x: X) -> Option<(usize, &P)> where
        P: Point<X, Y>,
        X: PartialOrd,
        Y: Spatial,
        Y::Scalar: Float,
    {
        self.point_idx_after(x).map(|i| (i, &self.points()[i]))
    }

    /// A reference to the first point that is equal to or comes after the given `x` along with
    /// its index.
    #[inline]
    fn point_on_or_after_with_idx<X, Y>(&self, x: X) -> Option<(usize, &P)> where
        P: Point<X, Y>,
        X: PartialOrd,
        Y: Spatial,
        Y::Scalar: Float,
    {
        self.point_idx_on_or_after(x).map(|i| (i, &self.points()[i]))
    }


    /// Return `y` for the given `x`.
    ///
    /// If there is less than two points interpolation is not meaningful,
    /// thus we should just return None.
    ///
    /// Note: It is assumed that the points owned by the Envelope are sorted by `x`.
    #[inline]
    fn y<X, Y>(&self, x: X) -> Option<Y>
        where
            X: PartialOrd,
            Y: Spatial + PartialEq,
            Y::Scalar: Float,
            P: Point<X, Y>,
    {
        let points = self.points();
        let len = points.len();

        // If we have less than two points, there is nothing to interpolate.
        if len < 2 {
            // However if we only have one point...
            if len == 1 {
                // And that point happens to be exactly equal to the given X.
                if points[0].x() == x {
                    // Return the Y at that given X.
                    return Some(points[0].y());
                }
            }
            return None;
        }
        
        // If the given `x` is less than our first point's `X` or greater than our last point's
        // `X`, we cannot interpolate the envelope and thus must return `None`.
        let last_idx = len - 1;
        if x < points[0].x() || x > points[last_idx].x() {
            return None;
        }

        // Otherwise, we know that X lies within our points and that we can interpolate it!
        let mut end_idx = 1;
        while end_idx < last_idx && x >= points[end_idx].x() {
            end_idx += 1;
        }
        let start_idx = end_idx - 1;
        Some(Point::interpolate(x, &points[start_idx], &points[end_idx]))
    }

    // /// An iterator yielding the X for each point at which the envelope intersects the given `y`.
    // ///
    // /// If there are any periods at which X is continuous, only the start X of the continuous
    // /// period will be returned.
    // fn xs_at_y(&self, y: Y) -> XsAtY {
    //     unimplemented!();
    // }

}

