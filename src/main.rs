use crate::log::init_logger;
use crate::protocol::message_type::MessageType;
use crate::protocol::Fit;
use binrw::BinResult;
use std::fs::read;
use tracing::info;

mod log;
mod protocol;

fn main() -> BinResult<()> {
    init_logger();
    let file = read("./tests/2015-06-09-21-12-06.fit").unwrap();
    let fit: Fit = Fit::read(file)?;
    for data in &fit.data {
        match data.message.message_type {
            MessageType::Record => {
                info!("Record: {:?}", data.message.values);
            }
            MessageType::None => {}
            _ => {}
        }
    }
    fit.write("./tests/test.fit")?;
    Ok(())
}
