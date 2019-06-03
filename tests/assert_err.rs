use assertive::assert_err;

#[test]
fn err() {
    let err = assert_err!(Err::<(), _>("hello"));
    assert_eq!(err, "hello");
}

#[test]
#[should_panic]
fn ok() {
    assert_err!(Ok::<_, ()>("boom"));
}
