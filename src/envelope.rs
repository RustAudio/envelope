
use interpolate::Scalar;
use interpolation::Spatial;
use point::Point;
use std;


/// Types that are representable as an Envelope.
pub trait Envelope<'a>: Sized {
    type X: PartialEq + PartialOrd + Clone;
    type Y: PartialEq + Spatial;
    /// The `Point` type which may be referenced and interpolated by the `Envelope`.
    type Point: Point<X=Self::X, Y=Self::Y> + 'a;
    /// An iterator yielding references to `Self::Point`s.
    type Points: Iterator<Item=&'a Self::Point>
        + ExactSizeIterator
        + DoubleEndedIterator
        + Clone
        + 'a;

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
    fn y(&'a self, x: Self::X) -> Option<Self::Y>
        where <Self::Y as Spatial>::Scalar: Scalar,
    {
        y(self, x)
    }

    /// Sample the `Envelope`'s `y` value for every given positive `x` step starting from the first
    /// point's `X` value.
    ///
    /// The envelope will yield `Some(Y)` until the first step is out of range of all points on the
    /// y axis.
    ///
    /// Returns `None` if `start` is outside the bounds of all points.
    ///
    /// Note: This method assumes that the envelope points are ordered.
    #[inline]
    fn steps(&'a self, start: Self::X, step: Self::X) -> Option<Steps<'a, Self>> {
        let mut points = self.points();
        points.next().and_then(|mut left| {
            let mut maybe_right = None;

            // Iterate through `points` until `start` is between `left` and `right`
            while let Some(point) = points.next() {
                maybe_right = Some(point);
                if point.x() < start {
                    left = maybe_right.take().unwrap();
                } else {
                    break;
                }
            }

            // Check that the remaining points bound the `start`.
            match maybe_right {
                Some(right) => if right.x() < start { return None; },
                None => if left.x() < start { return None; },
            }

            Some(Steps {
                points: points,
                step: step,
                next_x: start,
                left: left,
                maybe_right: maybe_right,
                env: std::marker::PhantomData,
            })
        })
    }

    // /// An iterator yielding the X for each point at which the envelope intersects the given `y`.
    // ///
    // /// If there are any periods at which X is continuous, only the start X of the continuous
    // /// period will be returned.
    // fn xs_at_y(&self, y: Y) -> XsAtY {
    //     unimplemented!();
    // }

}


/// An iterator that interpolates the envelope `E` one `step` and yields the result.
///
/// Returns `None` the first time `next` falls out of range of all points in `env`.
#[derive(Clone)]
pub struct Steps<'a, E>
    where E: Envelope<'a> + 'a,
{
    points: E::Points,
    step: E::X,
    next_x: E::X,
    left: &'a E::Point,
    maybe_right: Option<&'a E::Point>,
    env: std::marker::PhantomData<E>,
}

impl<'a, E> Steps<'a, E>
    where E: Envelope<'a>,
{
    /// This is useful when the step size must change between steps.
    #[inline]
    pub fn set_step(&mut self, step: E::X) {
        self.step = step;
    }

    /// Yields the next step along with its position along the step.
    #[inline]
    pub fn next_xy(&mut self) -> Option<(E::X, E::Y)>
        where Self: Iterator<Item=E::Y>,
    {
        let x = self.next_x.clone();
        self.next().map(|y| (x, y))
    }
}

impl<'a, E> Iterator for Steps<'a, E>
    where E: Envelope<'a>,
          <E as Envelope<'a>>::X: std::ops::Add<Output=<E as Envelope<'a>>::X>,
          <<E as Envelope<'a>>::Y as Spatial>::Scalar: Scalar,
{
    type Item = E::Y;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let Steps {
            ref step,
            ref mut points,
            ref mut next_x,
            ref mut left,
            ref mut maybe_right,
            ..
        } = *self;

        let x = next_x.clone();
        *next_x = x.clone() + step.clone();
        maybe_right.as_mut()
            .and_then(|right| {
                let x = x.clone();
                while x > right.x() {
                    *left = right;
                    *right = match points.next() {
                        Some(point) => point,
                        None => return None,
                    };
                }
                Some(Point::interpolate(x, *left, *right))
            })
            .or_else(|| if x == left.x() { Some(left.y()) } else { None })
    }
}


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
          <<E as Envelope<'a>>::Y as Spatial>::Scalar: Scalar,
{
    let mut points = env.points();
    points.next().and_then(|mut left| {
        let mut maybe_right = None;

        // Iterate through `points` until `x` is between `left` and `right`
        while let Some(point) = points.next() {
            maybe_right = Some(point);
            if point.x() < x {
                left = maybe_right.take().unwrap();
            } else {
                break;
            }
        }

        // Check that the remaining points bound the `x`.
        match maybe_right {
            Some(right) => if right.x() < x { return None; },
            None => if left.x() < x { return None; },
        }

        maybe_right
            .and_then(|mut right| {
                let x = x.clone();
                while x > right.x() {
                    left = right;
                    right = match points.next() {
                        Some(point) => point,
                        None => return None,
                    };
                }
                Some(Point::interpolate(x, left, right))
            })
            .or_else(|| if x == left.x() { Some(left.y()) } else { None })
    })
}
