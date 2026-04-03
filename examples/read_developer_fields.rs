use fit_rust::Fit;
use fit_rust::protocol::FitMessage;
use fit_rust::protocol::data_field::FieldNum;
use fit_rust::protocol::message_type::MessageType;
use std::fs;

fn main() {
    let file = fs::read("tests/test.fit").unwrap();
    let fit: Fit = Fit::read(file).unwrap();
    for data in &fit.data {
        match data {
            FitMessage::Definition(msg) => {
                println!("Definition: {:?}", msg.data);
            }
            FitMessage::Data(msg) if msg.data.message_type == MessageType::FileId => {
                println!("FileId data type: {:?}", msg.data);
            }
            FitMessage::Data(msg) if msg.data.message_type == MessageType::Record => {
                for v in &msg.data.values {
                    match v.field_num {
                        FieldNum::GlobalProfile(field_num) => {}
                        FieldNum::DeveloperData(dev_data_index, field_num) => {}
                    }
                }
            }
            FitMessage::Data(_) => {}
        }
    }
}
