use fit_rust::Fit;
use std::fs;

fn main() {
    let file = fs::read("test.fit").unwrap();
    let fit: Fit = Fit::read(file).unwrap();
    fit.write("test1.fit").unwrap();
}
