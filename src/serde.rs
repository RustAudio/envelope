extern crate serde;

use {BezierPoint, EasePoint};

// impl<X, Y> serde::Serialize for BezierPoint<X, Y>
//     where X: serde::Serialize,
//           Y: serde::Serialize,
//           Y::Scalar: serde::Serialize,
// {
//     fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
//         where S: serde::Serializer,
//     {
//         //serializer.serialize_struct("BezierPoint", 
//     }
// }
// 
// pub struct BezierPointMapVisitor<'a, X, Y> {
//     point: &BezierPoint<X, Y>,
//     field_n: u8,
// }
// 
// impl<'a, X, Y> serde::ser::MapVisitor for BezierPointMapVisitor<'a, X, Y> {
// 
// }
