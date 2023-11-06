use crate::log::init_logger;
use crate::protocol::message_type::MessageType;
use crate::protocol::FitReader;
use binrw::BinResult;
use std::fs::read;
use tracing::info;

mod log;
mod protocol;

fn main() -> BinResult<()> {
    init_logger();
    let file = read("./tests/2015-06-09-21-12-06.fit").unwrap();
    let fit: FitReader = FitReader::read(file)?;
    for data in &fit.data {
        match data.message_type {
            MessageType::Record => {
                info!("Record: {:?}", data.values);
            }
            MessageType::None => {}
            _ => {}
        }
    }
    // let write_path = "./tests/test.fit";
    // fit.write(write_path)?;
    Ok(())
}
