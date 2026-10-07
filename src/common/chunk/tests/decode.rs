use crate::common::chunk::Instr;

#[test]
fn instr_fits_in_eight_bytes() {
    assert!(std::mem::size_of::<Instr>() <= 8);
}
