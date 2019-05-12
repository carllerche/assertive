use assertive::assert_ok;

#[test]
fn ok() {
    let value = assert_ok!(Ok::<_, ()>("hello"));
    assert_eq!(value, "hello");
}

#[test]
#[should_panic]
fn err() {
    assert_ok!(Err::<(), _>("boom"));
}
