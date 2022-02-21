use crate::geometry::{Scalar, Vect};

#[test]
fn cross_product() {
    let v1 = Vect::new(6., 8., 8.);
    let v2 = Vect::new(3., 4., 5.);
    let x = v1 * v2;
    assert_eq!(Vect::new(8., -6., 0.), x);
}
#[test]
fn dot_product() {
    let v1 = Vect::new(6., 8., 8.);
    let v2 = Vect::new(3., 4., 5.);
    let x = v1.dot(v2);
    assert_eq!(90 as Scalar, x);
}
#[test]
fn distance() {
    let v1 = Vect::new(7., 4., 3.);
    let v2 = Vect::new(17., 6., 2.);
    let x = v1.euclid_distance(&v2);
    assert!((10.24..10.247).contains(&x));

    let v1 = Vect::new(1., 3., 0.);
    let v2 = Vect::new(5., 2., 0.);
    let x = v1.euclid_distance(&v2);
    assert!((4.12..4.13).contains(&x));

    let v1 = Vect::new(0., 0., 0.);
    let v2 = Vect::new(3., 3., 0.);
    let x = v1.euclid_distance(&v2);
    assert!((4.23..4.26).contains(&x));
}
