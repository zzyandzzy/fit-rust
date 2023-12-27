use fit_rust::Fit;

fn main() {
    Fit::merge(
        vec![
            "tests/ride-0-2023-09-29-12-49-21.fit",
            "tests/ride-0-2023-09-29-09-41-54.fit",
        ],
        "tests/merge.fit",
    )
    .unwrap();
}
