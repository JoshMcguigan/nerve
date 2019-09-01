// See `build.rs`
// One test is generated per program in the `bf-examples` directory
include!(concat!(env!("OUT_DIR"), "/gen_tests.rs"));
