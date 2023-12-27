use fit_rust::Fit;
use std::fs;

fn main() {
    let file = fs::read("tests/test.fit").unwrap();
    let fit: Fit = Fit::read(file).unwrap();
    fit.write("tests/write-test.fit").unwrap();
}
