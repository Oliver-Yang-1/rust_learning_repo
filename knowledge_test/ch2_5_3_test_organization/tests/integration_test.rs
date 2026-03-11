use ch2_5_3_test_organization::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, add_two(2));
}
