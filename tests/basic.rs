use fs_database::Rectangle;

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {width: 8, height: 7};
    let smaller = Rectangle {width:5, height: 1};

    assert!(larger.can_hold(&smaller));
}