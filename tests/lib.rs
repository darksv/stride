mod iter;
mod ops;

use stride::Stride;

#[test]
fn stride_debug() {
    let stride = Stride::<_, 1>::new(&[1, 2, 3, 4, 5]);
    assert_eq!(format!("{:?}", stride), "[1, 2, 3, 4, 5]");

    let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5]);
    assert_eq!(format!("{:?}", stride), "[1, 3, 5]");

    let stride = Stride::<_, 3>::new(&[1, 2, 3, 4, 5]);
    assert_eq!(format!("{:?}", stride), "[1, 4]");
}

#[test]
fn stride_default() {
    let stride: &Stride<i64, 3> = Default::default();
    assert_eq!(stride.len(), 0);
}

#[test]
fn stride_mut_default() {
    let stride: &mut Stride<i64, 3> = Default::default();
    assert_eq!(stride.len(), 0);
}

#[test]
fn stride_new_multiple() {
    let stride = Stride::<_, 3>::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(stride[0], 1);
    assert_eq!(stride[1], 4);
}

#[test]
fn stride_new_non_multiple() {
    let stride = Stride::<_, 3>::new(&[1, 2, 3, 4]);
    assert_eq!(stride[0], 1);
    assert_eq!(stride[1], 4);
}

#[test]
fn stride_len() {
    let stride = Stride::<_, 3>::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(stride.len(), 2);
}

#[test]
fn stride_len_non_multiple() {
    let stride = Stride::<_, 3>::new(&[1, 2, 3, 4, 5]);
    assert_eq!(stride.len(), 2);
}
