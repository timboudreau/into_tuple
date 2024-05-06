//! This crate provides a trait for converting rust constant-size *arrays* (not `Vec`s!) into tuples
//! with the same bound.  An esoteric problem, perhaps, but when you have it, you have it. It is useful
//! when using [const generics](https://doc.rust-lang.org/reference/items/generics.html#const-generics)
//! to support specific numbers of items.
//!
//! The `IntoTuple` trait will be available on arrays and borrows of arrays of any type in sizes up to the
//! bound set for this library at compile-time.  The default is tuples of up to 12 items.
//!
//! The boundary can be adjusted by building with certain (mutually exclusive) feature flags:
//!  * (none) - the default is up to 12 items
//!  * `medium` - up to 24 items
//!  * `large` - up to 64 items
//!  * `huge` - up to 250 items
//!
//! The larger sizes are likely only of utility in generated code, but they are available if you need them, but
//! can result in substantial build times.

mod generated_tests;
mod test;

/// To ensure clashing implementations cannot be created.
trait Sealed {}

/// A trait which applies to any Rust array with a dimension smaller than or equal to 12
/// elements, which will convert the array into a tuple of the same dimension.
#[allow(private_bounds)]
pub trait IntoTuple<const N: usize, E, T> : Sealed {
    /// Returns a tuple of the same dimension as the passed array.
    fn into_tuple(self) -> T;
}

// Generated implementations:

impl<E> Sealed for [E; 0]{ }
impl<'l, E> Sealed for &'l [E; 0]{ }
impl<E> IntoTuple<0, E, ()> for [E; 0] {
    fn into_tuple(self: [E; 0]) -> () {
        ()
    }
}

impl<E> Sealed for [E; 1]{ }
impl<'l, E> Sealed for &'l [E; 1]{ }
impl<E> IntoTuple<1, E, (E,)> for [E; 1] {
    fn into_tuple(self: [E; 1]) -> (E,) {
        let mut it = self.into_iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
            )
        }
    }
}

impl<E> Sealed for [E; 2]{ }
impl<'l, E> Sealed for &'l [E; 2]{ }
impl<E> IntoTuple<2, E, (E, E)> for [E; 2] {
    fn into_tuple(self: [E; 2]) -> (E, E) {
        let mut it = self.into_iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
            )
        }
    }
}

impl<E> Sealed for [E; 3]{ }
impl<'l, E> Sealed for &'l [E; 3]{ }
impl<E> IntoTuple<3, E, (E, E, E)> for [E; 3] {
    fn into_tuple(self: [E; 3]) -> (E, E, E) {
        let mut it = self.into_iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
            )
        }
    }
}

impl<E> Sealed for [E; 4]{ }
impl<'l, E> Sealed for &'l [E; 4]{ }
impl<E> IntoTuple<4, E, (E, E, E, E)> for [E; 4] {
    fn into_tuple(self: [E; 4]) -> (E, E, E, E) {
        let mut it = self.into_iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
            )
        }
    }
}

impl<E> Sealed for [E; 5]{ }
impl<'l, E> Sealed for &'l [E; 5]{ }
impl<E> IntoTuple<5, E, (E, E, E, E, E)> for [E; 5] {
    fn into_tuple(self: [E; 5]) -> (E, E, E, E, E) {
        let mut it = self.into_iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
                 it.next().unwrap_unchecked(), // 4
            )
        }
    }
}

impl<E> Sealed for [E; 6]{ }
impl<'l, E> Sealed for &'l [E; 6]{ }
impl<E> IntoTuple<6, E, (E, E, E, E, E, E)> for [E; 6] {
    fn into_tuple(self: [E; 6]) -> (E, E, E, E, E, E) {
        let mut it = self.into_iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
                 it.next().unwrap_unchecked(), // 4
                 it.next().unwrap_unchecked(), // 5
            )
        }
    }
}

impl<E> Sealed for [E; 7]{ }
impl<'l, E> Sealed for &'l [E; 7]{ }
impl<E> IntoTuple<7, E, (E, E, E, E, E, E, E)> for [E; 7] {
    fn into_tuple(self: [E; 7]) -> (E, E, E, E, E, E, E) {
        let mut it = self.into_iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
                 it.next().unwrap_unchecked(), // 4
                 it.next().unwrap_unchecked(), // 5
                 it.next().unwrap_unchecked(), // 6
            )
        }
    }
}

impl<E> Sealed for [E; 8]{ }
impl<'l, E> Sealed for &'l [E; 8]{ }
impl<E> IntoTuple<8, E, (E, E, E, E, E, E, E, E)> for [E; 8] {
    fn into_tuple(self: [E; 8]) -> (E, E, E, E, E, E, E, E) {
        let mut it = self.into_iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
                 it.next().unwrap_unchecked(), // 4
                 it.next().unwrap_unchecked(), // 5
                 it.next().unwrap_unchecked(), // 6
                 it.next().unwrap_unchecked(), // 7
            )
        }
    }
}

impl<E> Sealed for [E; 9]{ }
impl<'l, E> Sealed for &'l [E; 9]{ }
impl<E> IntoTuple<9, E, (E, E, E, E, E, E, E, E, E)> for [E; 9] {
    fn into_tuple(self: [E; 9]) -> (E, E, E, E, E, E, E, E, E) {
        let mut it = self.into_iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
                 it.next().unwrap_unchecked(), // 4
                 it.next().unwrap_unchecked(), // 5
                 it.next().unwrap_unchecked(), // 6
                 it.next().unwrap_unchecked(), // 7
                 it.next().unwrap_unchecked(), // 8
            )
        }
    }
}

impl<E> Sealed for [E; 10]{ }
impl<'l, E> Sealed for &'l [E; 10]{ }
impl<E> IntoTuple<10, E, (E, E, E, E, E, E, E, E, E, E)> for [E; 10] {
    fn into_tuple(self: [E; 10]) -> (E, E, E, E, E, E, E, E, E, E) {
        let mut it = self.into_iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
                 it.next().unwrap_unchecked(), // 4
                 it.next().unwrap_unchecked(), // 5
                 it.next().unwrap_unchecked(), // 6
                 it.next().unwrap_unchecked(), // 7
                 it.next().unwrap_unchecked(), // 8
                 it.next().unwrap_unchecked(), // 9
            )
        }
    }
}

impl<E> Sealed for [E; 11]{ }
impl<'l, E> Sealed for &'l [E; 11]{ }
impl<E> IntoTuple<11, E, (E, E, E, E, E, E, E, E, E, E, E)> for [E; 11] {
    fn into_tuple(self: [E; 11]) -> (E, E, E, E, E, E, E, E, E, E, E) {
        let mut it = self.into_iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
                 it.next().unwrap_unchecked(), // 4
                 it.next().unwrap_unchecked(), // 5
                 it.next().unwrap_unchecked(), // 6
                 it.next().unwrap_unchecked(), // 7
                 it.next().unwrap_unchecked(), // 8
                 it.next().unwrap_unchecked(), // 9
                 it.next().unwrap_unchecked(), // 10
            )
        }
    }
}

impl<E> Sealed for [E; 12]{ }
impl<'l, E> Sealed for &'l [E; 12]{ }
impl<E> IntoTuple<12, E, (E, E, E, E, E, E, E, E, E, E, E, E)> for [E; 12] {
    fn into_tuple(self: [E; 12]) -> (E, E, E, E, E, E, E, E, E, E, E, E) {
        let mut it = self.into_iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
                 it.next().unwrap_unchecked(), // 4
                 it.next().unwrap_unchecked(), // 5
                 it.next().unwrap_unchecked(), // 6
                 it.next().unwrap_unchecked(), // 7
                 it.next().unwrap_unchecked(), // 8
                 it.next().unwrap_unchecked(), // 9
                 it.next().unwrap_unchecked(), // 10
                 it.next().unwrap_unchecked(), // 11
            )
        }
    }
}

impl<'l, E> IntoTuple<1, E, (&'l E,)> for &'l [E; 1] {
    fn into_tuple(self: &'l [E; 1]) -> (&'l E,) {
        let mut it = self.iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
            )
        }
    }
}

impl<'l, E> IntoTuple<2, E, (&'l E, &'l E)> for &'l [E; 2] {
    fn into_tuple(self: &'l [E; 2]) -> (&'l E, &'l E) {
        let mut it = self.iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
            )
        }
    }
}

impl<'l, E> IntoTuple<3, E, (&'l E, &'l E, &'l E)> for &'l [E; 3] {
    fn into_tuple(self: &'l [E; 3]) -> (&'l E, &'l E, &'l E) {
        let mut it = self.iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
            )
        }
    }
}

impl<'l, E> IntoTuple<4, E, (&'l E, &'l E, &'l E, &'l E)> for &'l [E; 4] {
    fn into_tuple(self: &'l [E; 4]) -> (&'l E, &'l E, &'l E, &'l E) {
        let mut it = self.iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
            )
        }
    }
}

impl<'l, E> IntoTuple<5, E, (&'l E, &'l E, &'l E, &'l E, &'l E)> for &'l [E; 5] {
    fn into_tuple(self: &'l [E; 5]) -> (&'l E, &'l E, &'l E, &'l E, &'l E) {
        let mut it = self.iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
                 it.next().unwrap_unchecked(), // 4
            )
        }
    }
}

impl<'l, E> IntoTuple<6, E, (&'l E, &'l E, &'l E, &'l E, &'l E, &'l E)> for &'l [E; 6] {
    fn into_tuple(self: &'l [E; 6]) -> (&'l E, &'l E, &'l E, &'l E, &'l E, &'l E) {
        let mut it = self.iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
                 it.next().unwrap_unchecked(), // 4
                 it.next().unwrap_unchecked(), // 5
            )
        }
    }
}

impl<'l, E> IntoTuple<7, E, (&'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E)> for &'l [E; 7] {
    fn into_tuple(self: &'l [E; 7]) -> (&'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E) {
        let mut it = self.iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
                 it.next().unwrap_unchecked(), // 4
                 it.next().unwrap_unchecked(), // 5
                 it.next().unwrap_unchecked(), // 6
            )
        }
    }
}

impl<'l, E> IntoTuple<8, E, (&'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E)> for &'l [E; 8] {
    fn into_tuple(self: &'l [E; 8]) -> (&'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E) {
        let mut it = self.iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
                 it.next().unwrap_unchecked(), // 4
                 it.next().unwrap_unchecked(), // 5
                 it.next().unwrap_unchecked(), // 6
                 it.next().unwrap_unchecked(), // 7
            )
        }
    }
}

impl<'l, E> IntoTuple<9, E, (&'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E)> for &'l [E; 9] {
    fn into_tuple(self: &'l [E; 9]) -> (&'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E) {
        let mut it = self.iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
                 it.next().unwrap_unchecked(), // 4
                 it.next().unwrap_unchecked(), // 5
                 it.next().unwrap_unchecked(), // 6
                 it.next().unwrap_unchecked(), // 7
                 it.next().unwrap_unchecked(), // 8
            )
        }
    }
}

impl<'l, E> IntoTuple<10, E, (&'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E)> for &'l [E; 10] {
    fn into_tuple(self: &'l [E; 10]) -> (&'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E) {
        let mut it = self.iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
                 it.next().unwrap_unchecked(), // 4
                 it.next().unwrap_unchecked(), // 5
                 it.next().unwrap_unchecked(), // 6
                 it.next().unwrap_unchecked(), // 7
                 it.next().unwrap_unchecked(), // 8
                 it.next().unwrap_unchecked(), // 9
            )
        }
    }
}

impl<'l, E> IntoTuple<11, E, (&'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E)> for &'l [E; 11] {
    fn into_tuple(self: &'l [E; 11]) -> (&'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E) {
        let mut it = self.iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
                 it.next().unwrap_unchecked(), // 4
                 it.next().unwrap_unchecked(), // 5
                 it.next().unwrap_unchecked(), // 6
                 it.next().unwrap_unchecked(), // 7
                 it.next().unwrap_unchecked(), // 8
                 it.next().unwrap_unchecked(), // 9
                 it.next().unwrap_unchecked(), // 10
            )
        }
    }
}

impl<'l, E> IntoTuple<12, E, (&'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E)> for &'l [E; 12] {
    fn into_tuple(self: &'l [E; 12]) -> (&'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E, &'l E) {
        let mut it = self.iter();
        unsafe {
            (
                 it.next().unwrap_unchecked(), // 0
                 it.next().unwrap_unchecked(), // 1
                 it.next().unwrap_unchecked(), // 2
                 it.next().unwrap_unchecked(), // 3
                 it.next().unwrap_unchecked(), // 4
                 it.next().unwrap_unchecked(), // 5
                 it.next().unwrap_unchecked(), // 6
                 it.next().unwrap_unchecked(), // 7
                 it.next().unwrap_unchecked(), // 8
                 it.next().unwrap_unchecked(), // 9
                 it.next().unwrap_unchecked(), // 10
                 it.next().unwrap_unchecked(), // 11
            )
        }
    }
}

