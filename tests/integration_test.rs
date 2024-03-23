mod common;

use hello_world::adder;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add(2, 2));
}