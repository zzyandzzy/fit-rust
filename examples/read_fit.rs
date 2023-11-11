use binrw::BinResult;
use fit_rust::protocol::message_type::MessageType;
use fit_rust::Fit;
use std::fs;

fn main() -> BinResult<()> {
    let file = fs::read("test.fit").unwrap();
    let fit: Fit = Fit::read(file)?;
    for data in &fit.data {
        match data.message.message_type {
            MessageType::Record => {
                println!("Record: {:?}", data);
            }
            _ => {}
        }
    }
    Ok(())
}
