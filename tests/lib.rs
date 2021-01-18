use stride::Stride;

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
fn stride_len() {
    let stride = Stride::<_, 3>::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(stride.len(), 2);
}

#[test]
fn stride_index() {
    let stride = Stride::<_, 3>::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(stride[0], 1);
    assert_eq!(stride[1], 4);
}

#[test]
fn stride_index_mut() {
    let mut data = vec![1, 2, 3, 4, 5, 6];
    let stride = Stride::<_, 3>::new_mut(data.as_mut_slice());
    stride[0] = 7;
    stride[1] = 8;
    assert_eq!(stride[0], 7);
    assert_eq!(stride[1], 8);
}
