use fit_rust::Fit;
use std::fs;

fn main() {
    let file = fs::read("test.fit").unwrap();
    let fit: Fit = Fit::read(file).unwrap();
    for data in &fit.data {
        match data.message.message_type {
            _ => {
                println!("{:?}", data.message);
            }
        }
    }
}
