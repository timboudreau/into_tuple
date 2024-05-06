into_tuple
==========

A trivial Rust crate which provides a trait and implementations of it for converting Rust const-sized arrays
into Rust tuples, e.g.

```rust
let arr = [1_usize, 2_usize, 3_usize];

let tuple : (&usize, &usize, &usize) = arr.into_tuple();

assert_eq!((1_usize, 2_usize, 3_usize), tuple);
```

or with borrows

```rust
let arr = [1_usize, 2_usize, 3_usize];

let borrowed = &arr;

let tuple : (&usize, &usize, &usize) = borrowed.into_tuple();
```

The library has a hard maximum dimension which it will convert, since there need to be specific, generated
implementations for each possible dimension.  The default maximum is a size of 12.

The (mutually exclusive) feature flags `medium`, `large` and `huge` allow building it to support 24, 64 or 250
element arrays/tuples.  Given the readability of such code would be less than great, the larger sizes are mainly
of use in or calling other generated code.


Publishing on crates.io
-----------------------

While this was intended to be published there, I have yet to find a strategy that works to generate
the sources to a library, and not have `cargo publish` fail it, or have tests not able to `use` types
from the crate.  Pending.

License
-------

MIT license - do what thou wilt.
