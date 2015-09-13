
extern crate envelope;


use envelope::Envelope;
use envelope::interpolation::Spatial;


/// Implement a Point and Envelope for the given X and Y types.
macro_rules! impl_point_and_envelope {
    ($X:ty, $Y:ty) => {
        #[derive(Copy, Clone, Debug, PartialEq)]
        struct Point {
            x: $X,
            y: $Y,
        }
        impl envelope::Point<$X, $Y> for Point {
            fn x_to_scalar(x: $X) -> <$Y as Spatial>::Scalar { x as <$Y as Spatial>::Scalar }
            fn x(&self) -> $X { self.x }
            fn y(&self) -> $Y { self.y }
        }
        impl Envelope<Point> for Vec<Point> {
            fn from_points<I: IntoIterator<Item=Point>>(points: I) -> Self {
                use std::iter::FromIterator;
                Vec::from_iter(points)
            }
            fn points(&self) -> &[Point] { self }
        }
    };
}


/// Check that for the given types X and Y, `y` is accurate when X falls exactly upon a point.
/// Y is expected to always be some float type.
macro_rules! test_x_y_float {
    ($test_name:ident, $X:ty, $Y:ty) => {

        #[test]
        fn $test_name() {

            impl_point_and_envelope!($X, $Y);

            fn sine(x: $X) -> $Y {
                ((x as $Y).sin() / 2.0) + 0.5
            }

            // Values exactly on points.
            let points_a = Vec::from_points((0..1_000).map(|i| {
                let x = i as $X;
                let value = sine(x);
                Point { x: x, y: value } 
            }));
            for i in 0..1_000 {
                let x = i as $X;
                let value = sine(x);
                let y_at_x = match points_a.y(x) {
                    Some(y) => y,
                    None => panic!("Cannot interpolate x {:?}", x),
                };
                assert_eq!(value, y_at_x);
            }

            // Interpolation betwen points.
            let points_b = Vec::from_points(vec![
                Point { x: 0 as $X, y: 0.0 as $Y },
                Point { x: 10 as $X, y: 1.0 as $Y },
            ]);
            assert_eq!(points_b.y(0 as $X).expect("Cannot interpolate 0"), 0.0 as $Y);
            assert_eq!(points_b.y(1 as $X).expect("Cannot interpolate 1"), 0.1 as $Y);
            assert_eq!(points_b.y(2 as $X).expect("Cannot interpolate 2"), 0.2 as $Y);
            assert_eq!(points_b.y(3 as $X).expect("Cannot interpolate 3"), 0.3 as $Y);
            assert_eq!(points_b.y(4 as $X).expect("Cannot interpolate 4"), 0.4 as $Y);
            assert_eq!(points_b.y(5 as $X).expect("Cannot interpolate 5"), 0.5 as $Y);
            assert_eq!(points_b.y(6 as $X).expect("Cannot interpolate 6"), 0.6 as $Y);
            assert_eq!(points_b.y(7 as $X).expect("Cannot interpolate 7"), 0.7 as $Y);
            assert_eq!(points_b.y(8 as $X).expect("Cannot interpolate 8"), 0.8 as $Y);
            assert_eq!(points_b.y(9 as $X).expect("Cannot interpolate 9"), 0.9 as $Y);
            assert_eq!(points_b.y(10 as $X).expect("Cannot interpolate 10"), 1.0 as $Y);
            assert_eq!(points_b.y(11 as $X), None);

            // Finding points.
            assert_eq!(points_b.point_idx_before(5 as $X), Some(0));
            assert_eq!(points_b.point_idx_before(0 as $X), None);
            assert_eq!(points_b.point_idx_on_or_before(5 as $X), Some(0));
            assert_eq!(points_b.point_idx_on_or_before(0 as $X), Some(0));
            assert_eq!(points_b.point_idx_on_or_before(10 as $X), Some(1));

            assert_eq!(points_b.point_idx_after(5 as $X), Some(1));
            assert_eq!(points_b.point_idx_after(10 as $X), None);
            assert_eq!(points_b.point_idx_on_or_after(5 as $X), Some(1));
            assert_eq!(points_b.point_idx_on_or_after(10 as $X), Some(1));
            assert_eq!(points_b.point_idx_on_or_after(0 as $X), Some(0));

            assert_eq!(*points_b.point_before(5 as $X).unwrap(), points_b[0]);
            assert_eq!(*points_b.point_on_or_before(5 as $X).unwrap(), points_b[0]);
            assert_eq!(*points_b.point_on_or_before(0 as $X).unwrap(), points_b[0]);
            assert_eq!(*points_b.point_on_or_before(10 as $X).unwrap(), points_b[1]);

            assert_eq!(*points_b.point_after(5 as $X).unwrap(), points_b[1]);
            assert_eq!(*points_b.point_on_or_after(5 as $X).unwrap(), points_b[1]);
            assert_eq!(*points_b.point_on_or_after(0 as $X).unwrap(), points_b[0]);
            assert_eq!(*points_b.point_on_or_after(10 as $X).unwrap(), points_b[1]);
        }

    };
}


test_x_y_float!(xf32_yf32, f32, f32);
test_x_y_float!(xf32_yf64, f32, f64);

test_x_y_float!(xf64_yf32, f64, f32);
test_x_y_float!(xf64_yf64, f64, f64);

test_x_y_float!(xi32_yf32, i32, f32);
test_x_y_float!(xi32_yf64, i32, f64);

test_x_y_float!(xi64_yf32, i64, f32);
test_x_y_float!(xi64_yf64, i64, f64);

test_x_y_float!(xu32_yf32, u32, f32);
test_x_y_float!(xu32_yf64, u32, f64);

test_x_y_float!(xu64_yf32, u64, f32);
test_x_y_float!(xu64_yf64, u64, f64);


/// Check that for the given types X and Y, `y` is accurate when X falls exactly upon a point.
/// Y is expected to always be some float type.
macro_rules! test_x_y_int {
    ($test_name:ident, $X:ty, $Y:ty) => {

        #[test]
        fn $test_name() {

            impl_point_and_envelope!($X, $Y);

            fn sine(x: $X) -> $Y {
                (((x as f64).sin() * 20.0) + 10.0) as $Y
            }

            // Values exactly on points.
            let points_a = Vec::from_points((0..1_000).map(|i| {
                let x = i as $X;
                let value = sine(x);
                Point { x: x, y: value } 
            }));
            for i in 0..1_000 {
                let x = i as $X;
                let value = sine(x);
                let y_at_x = match points_a.y(x) {
                    Some(y) => y,
                    None => panic!("Cannot interpolate x {:?}", x),
                };
                assert_eq!(value, y_at_x);
            }

            // Interpolation betwen points.
            let points_b = Vec::from_points(vec![
                Point { x: 0 as $X, y: 0 as $Y },
                Point { x: 10 as $X, y: 100 as $Y },
            ]);
            assert_eq!(points_b.y(0 as $X).expect("Cannot interpolate 0"), 0 as $Y);
            assert_eq!(points_b.y(1 as $X).expect("Cannot interpolate 1"), 10 as $Y);
            assert_eq!(points_b.y(2 as $X).expect("Cannot interpolate 2"), 20 as $Y);
            assert_eq!(points_b.y(3 as $X).expect("Cannot interpolate 3"), 30 as $Y);
            assert_eq!(points_b.y(4 as $X).expect("Cannot interpolate 4"), 40 as $Y);
            assert_eq!(points_b.y(5 as $X).expect("Cannot interpolate 5"), 50 as $Y);
            assert_eq!(points_b.y(6 as $X).expect("Cannot interpolate 6"), 60 as $Y);
            assert_eq!(points_b.y(7 as $X).expect("Cannot interpolate 7"), 70 as $Y);
            assert_eq!(points_b.y(8 as $X).expect("Cannot interpolate 8"), 80 as $Y);
            assert_eq!(points_b.y(9 as $X).expect("Cannot interpolate 9"), 90 as $Y);
            assert_eq!(points_b.y(10 as $X).expect("Cannot interpolate 10"), 100 as $Y);
            assert_eq!(points_b.y(11 as $X), None);

            // Finding points.
            assert_eq!(points_b.point_idx_before(5 as $X), Some(0));
            assert_eq!(points_b.point_idx_before(0 as $X), None);
            assert_eq!(points_b.point_idx_on_or_before(5 as $X), Some(0));
            assert_eq!(points_b.point_idx_on_or_before(0 as $X), Some(0));
            assert_eq!(points_b.point_idx_on_or_before(10 as $X), Some(1));

            assert_eq!(points_b.point_idx_after(5 as $X), Some(1));
            assert_eq!(points_b.point_idx_after(10 as $X), None);
            assert_eq!(points_b.point_idx_on_or_after(5 as $X), Some(1));
            assert_eq!(points_b.point_idx_on_or_after(10 as $X), Some(1));
            assert_eq!(points_b.point_idx_on_or_after(0 as $X), Some(0));

            assert_eq!(*points_b.point_before(5 as $X).unwrap(), points_b[0]);
            assert_eq!(*points_b.point_on_or_before(5 as $X).unwrap(), points_b[0]);
            assert_eq!(*points_b.point_on_or_before(0 as $X).unwrap(), points_b[0]);
            assert_eq!(*points_b.point_on_or_before(10 as $X).unwrap(), points_b[1]);

            assert_eq!(*points_b.point_after(5 as $X).unwrap(), points_b[1]);
            assert_eq!(*points_b.point_on_or_after(5 as $X).unwrap(), points_b[1]);
            assert_eq!(*points_b.point_on_or_after(0 as $X).unwrap(), points_b[0]);
            assert_eq!(*points_b.point_on_or_after(10 as $X).unwrap(), points_b[1]);
        }

    };
}


test_x_y_int!(xf32_yi32, f32, i32);
test_x_y_int!(xf32_yi64, f32, i64);

test_x_y_int!(xf64_yi32, f64, i32);
test_x_y_int!(xf64_yi64, f64, i64);

test_x_y_int!(xi32_yi32, i32, i32);
test_x_y_int!(xi32_yi64, i32, i64);

test_x_y_int!(xi64_yi32, i64, i32);
test_x_y_int!(xi64_yi64, i64, i64);

test_x_y_int!(xu32_yi32, u32, i32);
test_x_y_int!(xu32_yi64, u32, i64);

test_x_y_int!(xu64_yi32, u64, i32);
test_x_y_int!(xu64_yi64, u64, i64);

