use binrw::BinResult;
use fit_rust::protocol::message_type::MessageType;
use fit_rust::Fit;
use std::fs::read;

fn main() -> BinResult<()> {
    let file = read("./tests/garmin_1000.fit").unwrap();
    let fit: Fit = Fit::read(file)?;
    for data in &fit.data {
        match data.message.message_type {
            MessageType::Record => {
                println!("{:?}", data.message.values);
            }
            _ => {}
        }
    }

    Ok(())
}
