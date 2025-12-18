use crate::common::Constants;
use crate::{as_number, number};

#[test]
fn value_can_be_written_to_constants() {
    let mut constants = Constants::new();
    constants.write_value(number!(123.45));

    assert_eq!(1, constants.len());
    assert_eq!(123.45, as_number!(constants.read_value(0)));
}

#[test]
fn value_can_be_read_to_constants() {
    let mut constants = Constants::new();
    constants.write_value(number!(123.45));

    assert_eq!(1, constants.len());
    assert_eq!(123.45, as_number!(constants.read_value(0)));
}
