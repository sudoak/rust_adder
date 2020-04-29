extern crate mylib;

use mylib::add_two;

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}