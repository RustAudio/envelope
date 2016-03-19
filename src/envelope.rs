
use interpolate::Scalar;
use interpolation::Spatial;
use point::Point;
use std;


/// Types that are representable as an Envelope.
pub trait Envelope<'a>: Sized {
    type Scalar: Scalar;
    type X: PartialEq + PartialOrd + Clone;
    type Y: PartialEq + Spatial<Scalar=Self::Scalar>;
    /// The `Point` type which may be referenced and interpolated by the `Envelope`.
    type Point: Point<Scalar=Self::Scalar, X=Self::X, Y=Self::Y> + 'a;
    /// An iterator yielding references to `Self::Point`s.
    type Points:
        Iterator<Item=&'a Self::Point> + ExactSizeIterator + DoubleEndedIterator + Clone + 'a;

    /// An iterator yielding the `Point`s of the Envelope.
    fn points(&'a self) -> Self::Points;

    /// The index of the `Point` that comes directly before the given `x`.
    #[inline]
    fn point_idx_before(&'a self, x: Self::X) -> Option<usize> {
        point_idx_before(self, x)
    }

    /// The index of the `Point` that either lands on or comes directly before the given `x`.
    #[inline]
    fn point_idx_on_or_before(&'a self, x: Self::X) -> Option<usize> {
        point_idx_on_or_before(self, x)
    }

    /// The index of the `Point` that comes directly after the given `x`.
    #[inline]
    fn point_idx_after(&'a self, x: Self::X) -> Option<usize> {
        point_idx_after(self, x)
    }

    /// The index of the `Point` that comes directly after the given `x`.
    #[inline]
    fn point_idx_on_or_after(&'a self, x: Self::X) -> Option<usize> {
        point_idx_on_or_after(self, x)
    }

    /// A reference to the first point that comes before the given `x`.
    #[inline]
    fn point_before(&'a self, x: Self::X) -> Option<&'a Self::Point> {
        self.point_idx_before(x).and_then(|i| self.points().nth(i))
    }

    /// A reference to the first point that is equal to or comes before the given `x`.
    #[inline]
    fn point_on_or_before(&'a self, x: Self::X) -> Option<&'a Self::Point> {
        self.point_idx_on_or_before(x).and_then(|i| self.points().nth(i))
    }

    /// A reference to the first point that comes before the given `x` along with its index.
    #[inline]
    fn point_before_with_idx(&'a self, x: Self::X) -> Option<(usize, &'a Self::Point)> {
        self.point_idx_before(x).and_then(|i| self.points().nth(i).map(|p| (i, p)))
    }

    /// A reference to the first point that is equal to or comes before the given `x` along with
    /// its index.
    #[inline]
    fn point_on_or_before_with_idx(&'a self, x: Self::X) -> Option<(usize, &'a Self::Point)> {
        self.point_idx_on_or_before(x).and_then(|i| self.points().nth(i).map(|p| (i, p)))
    }

    /// A reference to the first point that comes after the given `x`.
    #[inline]
    fn point_after(&'a self, x: Self::X) -> Option<&'a Self::Point> {
        self.point_idx_after(x).and_then(|i| self.points().nth(i))
    }

    /// A reference to the first point that is equal to or comes after the given `x`.
    #[inline]
    fn point_on_or_after(&'a self, x: Self::X) -> Option<&'a Self::Point> {
        self.point_idx_on_or_after(x).and_then(|i| self.points().nth(i))
    }

    /// A reference to the first point that comes after the given `x` along with its index.
    #[inline]
    fn point_after_with_idx(&'a self, x: Self::X) -> Option<(usize, &'a Self::Point)> {
        self.point_idx_after(x).and_then(|i| self.points().nth(i).map(|p| (i, p)))
    }

    /// A reference to the first point that is equal to or comes after the given `x` along with
    /// its index.
    #[inline]
    fn point_on_or_after_with_idx(&'a self, x: Self::X) -> Option<(usize, &'a Self::Point)> {
        self.point_idx_on_or_after(x).and_then(|i| self.points().nth(i).map(|p| (i, p)))
    }

    /// A reference to the first point lying directly on the given `x` if there is one.
    #[inline]
    fn point_at(&'a self, x: Self::X) -> Option<&'a Self::Point> {
        self.points().find(|p| p.x() == x)
    }

    /// A reference to the first point (along with it's index) lying directly on the given `x` if
    /// there is one.
    #[inline]
    fn point_at_with_idx(&'a self, x: Self::X) -> Option<(usize, &'a Self::Point)> {
        self.points().enumerate().find(|&(_, p)| p.x() == x)
    }

    /// The points that lie on either side of the given `x`.
    ///
    /// FIXME: This could be much faster.
    #[inline]
    fn surrounding_points(&'a self, x: Self::X)
        -> (Option<&'a Self::Point>, Option<&'a Self::Point>)
    {
        (self.point_on_or_before(x.clone()), self.point_after(x))
    }

    /// A reference point that is closest to the given `x` if there is one.
    ///
    /// FIXME: This could be much faster.
    #[inline]
    fn closest_point(&'a self, x: Self::X) -> Option<&'a Self::Point>
        where <Self as Envelope<'a>>::X: std::ops::Sub<Output=<Self as Envelope<'a>>::X>,
    {
        match self.surrounding_points(x.clone()) {
            (Some(before), Some(after)) =>
                if x.clone() - before.x() < after.x() - x { Some(before) } else { Some(after) },
            (Some(point), None) | (None, Some(point)) => Some(point),
            (None, None) => None,
        }
    }

    /// Return `y` for the given `x`.
    ///
    /// If there is less than two points interpolation is not meaningful,
    /// thus we should just return None.
    ///
    /// Note: It is assumed that the points owned by the Envelope are sorted by `x`.
    #[inline]
    fn y(&'a self, x: Self::X) -> Option<Self::Y> {
        y(self, x)
    }

    // /// Sample the `Envelope`'s `y` value for every given `x` step starting from the first point's
    // /// `X` value.
    // ///
    // /// The envelope will yield `Some(Y)` until the first step is out of range of all points on the
    // /// y axis.
    // fn step<X, Y>(&'a self, step: X) -> Step<Self, X, Y>
    //     where 
    // {
    // }

    // /// An iterator yielding the X for each point at which the envelope intersects the given `y`.
    // ///
    // /// If there are any periods at which X is continuous, only the start X of the continuous
    // /// period will be returned.
    // fn xs_at_y(&self, y: Y) -> XsAtY {
    //     unimplemented!();
    // }

}


// pub struct Step<E, X, Y>
// {
// }



#[inline]
fn point_idx_before<'a, E>(env: &'a E, x: E::X) -> Option<usize>
    where E: Envelope<'a>,
{
    env.points().enumerate()
        .take_while(|&(_, point)| point.x() < x )
        .last()
        .map(|(i, _)| i)
}


#[inline]
fn point_idx_on_or_before<'a, E>(env: &'a E, x: E::X) -> Option<usize>
    where E: Envelope<'a>,
{
    env.points().enumerate()
        .take_while(|&(_, point)| point.x() <= x )
        .last()
        .map(|(i, _)| i)
}


#[inline]
fn point_idx_after<'a, E>(env: &'a E, x: E::X) -> Option<usize>
    where E: Envelope<'a>,
{
    env.points().enumerate().rev()
        .take_while(|&(_, point)| point.x() > x )
        .last()
        .map(|(i, _)| i)
}


#[inline]
fn point_idx_on_or_after<'a, E>(env: &'a E, x: E::X) -> Option<usize>
    where E: Envelope<'a>,
{
    env.points().enumerate().rev()
        .take_while(|&(_, point)| point.x() >= x )
        .last()
        .map(|(i, _)| i)
}


#[inline]
fn y<'a, E>(env: &'a E, x: E::X) -> Option<E::Y>
    where E: Envelope<'a>,
          E::Y: Spatial + PartialEq + 'a,
{

    let mut points: E::Points = env.points();
    let len = points.len();

    // If we have less than two points, there is nothing to interpolate.
    if len < 2 {
        // However if we only have one point...
        if len == 1 {
            // And that point happens to be exactly equal to the given X.
            let only_point: &E::Point = points.clone().next().unwrap();
            if only_point.x() == x {
                // Return the Y at that given X.
                return Some(only_point.y());
            }
        }
        return None;
    }

    // If the given `x` is less than our first point's `X` or greater than our last point's
    // `X`, we cannot interpolate the envelope and thus must return `None`.
    let first_point: &E::Point = points.clone().next().unwrap();
    let last_point: &E::Point = points.clone().last().unwrap();
    if x < first_point.x() || x > last_point.x() {
        return None;
    }

    // Otherwise, we know that X lies within our points and that we can interpolate it!
    let mut end_idx = 1;
    let last_idx = len - 1;
    let mut end_points = points.clone().skip(1);
    while end_idx < last_idx {
        if x >= end_points.next().unwrap().x() {
            end_idx += 1;
        } else {
            break;
        }
    }

    let start_idx = end_idx - 1;
    let start_point = points.clone().nth(start_idx).unwrap();
    let end_point = points.nth(end_idx).unwrap();
    Some(Point::interpolate(x, start_point, end_point))
}

