use fit_rust::Fit;

fn main() {
    Fit::merge(vec!["test1.fit", "test2.fit", "test3.fit"], "test.fit").unwrap();
}
