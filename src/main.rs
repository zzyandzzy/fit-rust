use crate::protocol::message_type::MessageType;
use crate::protocol::value::Value;
use crate::protocol::{Fit, FitDataMessage};
use binrw::BinResult;
use std::fs::read;
mod protocol;

fn main() -> BinResult<()> {
    // let file = read("./tests/ride-1-2023-11-07-21-20-25.fit").unwrap();
    // let fit: Fit = Fit::read(file)?;
    // for data in &fit.data {
    //     match data.message.message_type {
    //         MessageType::Record => {
    //             // print_record(data);
    //         }
    //         //         MessageType::None => {}
    //         f => {
    //             info!("{:?}: {:?}", f, data.message.values);
    //         }
    //     }
    // }
    // fit.write("./tests/ride-1.fit")?;

    Fit::merge(
        vec![
            "./tests/ride-0-2023-09-29-09-41-54.fit",
            "./tests/ride-0-2023-09-29-12-49-21.fit",
            "./tests/ride-0-2023-09-29-18-57-47.fit",
        ],
        "./tests/merge.fit",
    )?;

    Ok(())
}

#[allow(unused)]
fn print_record(data: &FitDataMessage) {
    let mut timestamp = 0;
    let mut lat = 0_f32;
    let mut long = 0_f32;
    let mut alt = 0;
    let mut heart = 0;
    let mut cadence = 0;
    let mut distance = 0;
    let mut speed = 0;
    let mut power = 0;
    let mut temp = 0;
    for item in &data.message.values {
        match item.field_num {
            0 => lat = <Value as Into<f32>>::into(item.value.clone().unwrap()),
            1 => long = <Value as Into<f32>>::into(item.value.clone().unwrap()),
            2 => alt = <Value as Into<u16>>::into(item.value.clone().unwrap()),
            3 => heart = <Value as Into<u8>>::into(item.value.clone().unwrap()),
            4 => cadence = <Value as Into<u8>>::into(item.value.clone().unwrap()),
            5 => distance = <Value as Into<u32>>::into(item.value.clone().unwrap()),
            6 => speed = <Value as Into<u16>>::into(item.value.clone().unwrap()),
            7 => power = <Value as Into<u16>>::into(item.value.clone().unwrap()),
            13 => temp = <Value as Into<i8>>::into(item.value.clone().unwrap()),
            253 => timestamp = <Value as Into<u32>>::into(item.value.clone().unwrap()),
            _ => {}
        }
    }
    let distance = distance as f32 / 100000.0;
    let alt = alt as f32 / 5.0 - 500.0;
    let speed = speed as f32 / 1000.0 * 3.6;

    println!("{{timestamp: {}, lat: {:0<10}, long: {:0<10}, alt: {:.2}m, heart: {:>3}bpm, cadence: {:>3}rpm, distance: {:.4}km, speed: {:.2}km/h, power: {:>4}w, temp: {:>2}C}},",
                     timestamp,
                     lat, long,
                     alt, heart,
                     cadence, distance,
                     speed, power, temp);
}
