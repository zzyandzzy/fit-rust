use fit_rust::protocol::FitMessage;
use fit_rust::Fit;
use std::fs;

fn main() {
    let file = fs::read("tests/test.fit").unwrap();
    let fit: Fit = Fit::read(file).unwrap();
    for data in &fit.data {
        match data {
            FitMessage::Definition(msg) => {
                println!("Definition: {:?}", msg.data);
            }
            FitMessage::Data(msg) => {
                println!("Data: {:?}", msg.data);
            }
        }
    }
}
