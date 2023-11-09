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
    let file = read("./tests/ride-1-2023-11-07-21-20-25.fit").unwrap();
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
    fit.write("./tests/test1.fit")?;

    // Fit::merge(
    //     vec![
    //         "./tests/ride-0-2023-09-29-09-41-54.fit",
    //         "./tests/ride-0-2023-09-29-12-49-21.fit",
    //         "./tests/ride-0-2023-09-29-18-57-47.fit",
    //     ],
    //     "./tests/merge.fit",
    // )?;

    Ok(())
}
