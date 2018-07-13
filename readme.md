## TODOs

HAL:

1. Setup timer.rs.
2. review clock.rs, clock_generator macro. There should be 1 more clock available

## SVD2RUST

To generate the SVD crate, use the following:

`svd2rust -i ATSAMD09D14A.svd --nightly`
`form -i lib.rs -o atsamd09d14a/`
`cd atsamd09d14a; cargo fmt`

If `cargo fmt` fails with a missing binary, run the following:

`rustup component add rustfmt-preview`

