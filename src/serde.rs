extern crate serde;

// mod bezier_point {
//     use bezier_point::BezierPoint;
//     use interpolation::Spatial;
//     use num::Float;
// 
//     impl_serde_serialization!(struct BezierPoint {
//         generics: [X, Y],
//         where: {
//             X: [Clone, Copy],
//             Y: [Spatial, Clone, Copy],
//             <Y as Spatial>::Scalar: [Float],
//         },
//         fields: {
//             0 => x: X,
//             1 => y: Y,
//             2 => curve: Y::Scalar,
//         },
//     });
// 
//     #[test]
//     fn test() {
//         extern crate serde_json;
// 
//         let point = BezierPoint { x: 42, y: 5.0, curve: 0.0 };
//         let serialized = serde_json::to_string(&point).unwrap();
// 
//         println!("{}", serialized);
//         assert_eq!("{\"x\":42,\"y\":5,\"curve\":0}", &serialized);
//         
//         let deserialized: BezierPoint<i32, f32> = serde_json::from_str(&serialized).unwrap();
// 
//         println!("{:?}", deserialized);
//         assert_eq!(point, deserialized);
//     }
// 
// }

mod bezier_point {
    use bezier_point::BezierPoint;
    use interpolation::Spatial;
    use num::Float;
    use std;
    use super::serde;

    impl<X, Y> serde::Serialize for BezierPoint<X, Y>
        where X: Clone + Copy + serde::Serialize,
              Y: Spatial + Clone + Copy + serde::Serialize,
              <Y as Spatial>::Scalar: Float + serde::Serialize,
    {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer,
        {

            struct MapVisitor<'a, X, Y>
                where X: Clone + Copy + 'a,
                      Y: Spatial + Clone + Copy + 'a,
                      Y::Scalar: Float,
            {
                t: &'a BezierPoint<X, Y>,
                field_idx: u8,
            }

            impl<'a, X, Y> serde::ser::MapVisitor for MapVisitor<'a, X, Y>
                where X: Clone + Copy + serde::Serialize,
                      Y: Spatial + Clone + Copy + serde::Serialize,
                      Y::Scalar: Float + serde::Serialize,
            {
                fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
                    where S: serde::Serializer,
                {
                    match self.field_idx {
                        0 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_struct_elt("x", &self.t.x))))
                        },
                        1 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_struct_elt("y", &self.t.y))))
                        },
                        2 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_struct_elt("curve", &self.t.curve))))
                        },
                        _ => Ok(None),
                    }
                }
            }

            serializer.serialize_struct("BezierPoint", MapVisitor {
                t: self,
                field_idx: 0,
            })
        }
    }

    impl<X, Y> serde::Deserialize for BezierPoint<X, Y>
        where X: Clone + Copy + serde::Deserialize,
              Y: Spatial + Clone + Copy + serde::Deserialize,
              <Y as Spatial>::Scalar: Float + serde::Deserialize,
    {
        fn deserialize<D>(deserializer: &mut D) -> Result<BezierPoint<X, Y>, D::Error>
            where D: serde::de::Deserializer
        {

            struct Visitor<X, Y> {
                x: std::marker::PhantomData<X>,
                y: std::marker::PhantomData<Y>,
            }

            impl<X, Y> serde::de::Visitor for Visitor<X, Y>
                where X: Clone + Copy + serde::Deserialize,
                      Y: Spatial + Clone + Copy + serde::Deserialize,
                      <Y as Spatial>::Scalar: Float + serde::Deserialize,
            {
                type Value = BezierPoint<X, Y>;

                fn visit_map<V>(&mut self, mut visitor: V) -> Result<BezierPoint<X, Y>, V::Error>
                    where V: serde::de::MapVisitor,
                {

                    enum Field { X, Y, Curve }

                    impl serde::Deserialize for Field {
                        fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
                            where D: serde::de::Deserializer,
                        {
                            struct FieldVisitor;

                            impl serde::de::Visitor for FieldVisitor {
                                type Value = Field;

                                fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                                    where E: serde::de::Error,
                                {
                                    match value {
                                        "x" => Ok(Field::X),
                                        "y" => Ok(Field::Y),
                                        "curve" => Ok(Field::Curve),
                                        _ => Err(serde::de::Error::custom("expected x, y or curve")),
                                    }
                                }
                            }

                            deserializer.deserialize(FieldVisitor)
                        }
                    }

                    let mut x = None;
                    let mut y = None;
                    let mut curve = None;

                    loop {
                        match try!(visitor.visit_key()) {
                            Some(Field::X) => { x = Some(try!(visitor.visit_value())); },
                            Some(Field::Y) => { y = Some(try!(visitor.visit_value())); },
                            Some(Field::Curve) => { curve = Some(try!(visitor.visit_value())); },
                            None => { break; }
                        }
                    }

                    let x = match x {
                        Some(x) => x,
                        None => try!(visitor.missing_field("x")),
                    };

                    let y = match y {
                        Some(y) => y,
                        None => try!(visitor.missing_field("y")),
                    };

                    let curve = match curve {
                        Some(curve) => curve,
                        None => try!(visitor.missing_field("curve")),
                    };

                    try!(visitor.end());

                    Ok(BezierPoint { x: x, y: y, curve: curve })
                }
            }

            static FIELDS: &'static [&'static str] = &["x", "y", "curve"];
            let visitor = Visitor {
                x: std::marker::PhantomData,
                y: std::marker::PhantomData,
            };
            deserializer.deserialize_struct("BezierPoint", FIELDS, visitor)
        }
    }


    #[test]
    fn test() {
        extern crate serde_json;

        let point = BezierPoint { x: 42, y: 5.0, curve: 0.0 };
        let serialized = serde_json::to_string(&point).unwrap();

        println!("{}", serialized);
        assert_eq!("{\"x\":42,\"y\":5,\"curve\":0}", &serialized);
        
        let deserialized: BezierPoint<i32, f32> = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(point, deserialized);
    }
}
