//! Trivial tests - the only non-generated source in this library, which
//! exist mainly to cause a build failure if something doesn't exist which should.
#![cfg(test)]

use super::*;

const STATIC_ARR: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
static STATIC_REF: &'static [u8; 4] = &STATIC_ARR;

#[test]
fn test_borrow_no_lifetime() {
    let arr: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
    let aref: &[u8; 4] = &arr;
    let tup = aref.into_tuple();
    assert_eq!(1, *tup.0);
    assert_eq!(2, *tup.1);
    assert_eq!(3, *tup.2);
    assert_eq!(4, *tup.3);
}

#[test]
fn test_borrow_static() {
    let tup = STATIC_REF.into_tuple();
    assert_eq!(1, *tup.0);
    assert_eq!(2, *tup.1);
    assert_eq!(3, *tup.2);
    assert_eq!(4, *tup.3);
}
