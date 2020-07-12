extern crate lib;

// build creates -> rustc --crate-type=lib test.rs
// run commnad -> rustc main.rs --extern lib=libtest.rlib && ./main
fn main() {
    lib::test1();
    lib::test2();
}