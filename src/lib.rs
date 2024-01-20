pub mod protocol;

use crate::protocol::io::{skip_bytes, write_bin};
use crate::protocol::macros::get_field_value;
use crate::protocol::message_type::MessageType;
use crate::protocol::value::Value;
use crate::protocol::{
    calculate_fit_crc, DataMessage, DefinitionMessage, FitDataMessage, FitDefinitionMessage,
    FitHeader, FitMessage, FitMessageHeader,
};
use binrw::{BinReaderExt, BinResult, BinWrite, Endian, Error};
use std::collections::VecDeque;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs::{read, write};
use std::io::{Cursor, Seek, SeekFrom, Write};
use std::ops::Div;
use std::path::Path;

#[derive(Clone)]
pub struct Fit {
    pub header: FitHeader,

    pub data: Vec<FitMessage>,
}

impl Debug for Fit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Fit").field("header", &self.header).finish()
    }
}

impl Fit {
    pub fn read(buf: Vec<u8>) -> BinResult<Self> {
        let mut cursor = Cursor::new(buf);
        let header: FitHeader = cursor.read_ne()?;
        let mut queue: VecDeque<(u8, FitDefinitionMessage)> = VecDeque::new();

        let mut data: Vec<FitMessage> = Vec::new();
        loop {
            let message_header: FitMessageHeader = cursor.read_ne()?;
            match message_header.definition {
                true => {
                    if message_header.dev_fields {
                        unimplemented!("message_header.dev_fields is unimplemented");
                    }
                    let definition_message: DefinitionMessage =
                        cursor.read_ne_args((message_header.dev_fields,))?;

                    let local_num = message_header.local_num;
                    let def = FitDefinitionMessage {
                        header: message_header,
                        data: definition_message,
                    };
                    data.push(FitMessage::Definition(def.clone()));
                    queue.push_front((local_num, def));
                }
                false => {
                    let definition = match queue.iter().find(|x| x.0 == message_header.local_num) {
                        None => continue,
                        Some((_, def)) => def,
                    };
                    let data_message: DataMessage = cursor.read_ne_args((definition,))?;
                    if data_message.message_type == MessageType::None {
                        continue;
                    }
                    data.push(FitMessage::Data(FitDataMessage {
                        header: message_header,
                        data: data_message,
                    }));
                    if cursor.position() >= (header.data_size + header.header_size as u32) as u64 {
                        break;
                    }
                }
            }
        }
        Ok(Fit { header, data })
    }

    pub fn write<P: AsRef<Path>>(&self, file: P) -> BinResult<()> {
        let mut buf = Vec::with_capacity(
            (self.header.data_size + self.header.header_size as u32 + 2) as usize,
        );
        let header = self.write_buf(&mut buf)?;
        Fit::write_crc(header, &mut buf)?;
        write(file, &buf)?;
        Ok(())
    }

    fn write_crc(header: FitHeader, buf: &mut Vec<u8>) -> BinResult<()> {
        let mut header_crc: Option<u16> = None;
        if header.crc.is_some() {
            let header = &buf[0..(header.header_size - 2) as usize];
            header_crc = Some(calculate_fit_crc(&header));
        }
        let end_byte = header.header_size as u32 + header.data_size;
        let body = &buf[header.header_size as usize..end_byte as usize];
        let body_crc = calculate_fit_crc(&body);
        let mut writer = Cursor::new(buf);
        match header_crc {
            None => {}
            Some(crc) => {
                writer.seek(SeekFrom::Start(header.header_size as u64 - 2))?;
                write_bin(&mut writer, crc, Endian::Little)?;
            }
        }
        writer.seek(SeekFrom::End(0))?;
        write_bin(&mut writer, body_crc, Endian::Little)?;
        writer.flush()?;
        Ok(())
    }

    pub(crate) fn write_buf(&self, buf: &mut Vec<u8>) -> BinResult<FitHeader> {
        let mut queue: VecDeque<(u8, FitDefinitionMessage)> = VecDeque::new();
        let mut writer = Cursor::new(buf);
        skip_bytes(&mut writer, self.header.header_size);
        for massage in &self.data {
            match massage {
                FitMessage::Definition(msg) => {
                    msg.header.write(&mut writer)?;
                    msg.data.write(&mut writer)?;
                    let local_num = msg.header.local_num;
                    queue.push_front((local_num, msg.clone()));
                }
                FitMessage::Data(msg) => {
                    let message_header = &msg.header;
                    let definition = match queue.iter().find(|x| x.0 == message_header.local_num) {
                        None => None,
                        Some((_, def)) => Some(def),
                    };
                    match definition {
                        None => {}
                        Some(def) => {
                            if &msg.data.message_type != &MessageType::None {
                                msg.header.write(&mut writer)?;
                                msg.data.write(&mut writer, &def.data)?;
                            }
                        }
                    }
                }
            }
        }
        let mut header = self.header.clone();
        header.data_size = writer.position() as u32 - header.header_size as u32;
        writer.seek(SeekFrom::Start(0))?;
        header.write(&mut writer)?;
        writer.flush()?;
        Ok(header)
    }
}

impl Fit {
    #[allow(unused)]
    pub fn merge<P: AsRef<Path>>(files: Vec<P>, path: P) -> BinResult<()> {
        if files.is_empty() || files.len() <= 1 {
            eprintln!("Error files is empty: {:?}", files.len());
            return Err(Error::Io(binrw::io::Error::new(
                binrw::io::ErrorKind::UnexpectedEof,
                "Error files is empty!",
            )));
        }
        let file = read(files.get(0).unwrap()).unwrap();
        let mut fit: Fit = Fit::read(file)?;
        // find session
        let session: Option<(usize, FitDataMessage)> = fit.get_session();
        let mut sessions: Vec<Option<(usize, FitDataMessage)>> = vec![session];
        for i in 1..=files.len() - 1 {
            let f = files.get(i).unwrap();
            let f = read(f).unwrap();
            let mut tmp = Fit::read(f)?;
            sessions.push(tmp.get_session());

            let to_move: Vec<_> = tmp
                .data
                .iter()
                .enumerate()
                .filter(|(_, message)| match message {
                    FitMessage::Definition(_) => false,
                    FitMessage::Data(msg) => {
                        return matches!(msg.data.message_type, MessageType::Record);
                    }
                })
                .map(|(i, _)| i)
                .collect();

            for i in to_move.into_iter().rev() {
                // Here, we directly take the message without wrapping it in an Option.
                let message = tmp.data.swap_remove(i);
                fit.data.push(message);
            }
        }

        fit.replace_session(sessions);
        fit.write(path)
    }

    fn replace_session(&mut self, sessions: Vec<Option<(usize, FitDataMessage)>>) {
        let mut index = 0;
        let mut session_vec = vec![];
        for session in sessions {
            match session {
                None => {}
                Some((i, s)) => {
                    if index == 0 {
                        index = i;
                    }
                    session_vec.push(s);
                }
            }
        }

        let session = Fit::merge_sessions(session_vec);
        match session {
            None => {}
            Some(session) => {
                self.data[index] = FitMessage::Data(session);
            }
        }
    }

    fn merge_sessions(mut sessions: Vec<FitDataMessage>) -> Option<FitDataMessage> {
        if sessions.is_empty() {
            return None;
        }
        let mut merged_session = sessions.remove(0);
        // max
        let mut max_stop_timestamp = Value::Time(u32::MIN);
        let mut max_speed = Value::U16(u16::MIN);
        let mut max_power = Value::U16(u16::MIN);
        let mut max_altitude = Value::U16(u16::MIN);
        let mut max_pos_grade = Value::I16(i16::MIN);
        let mut max_neg_grade = Value::I16(i16::MIN);
        let mut max_heart_rate = Value::U8(u8::MIN);
        let mut max_cadence = Value::U8(u8::MIN);
        let mut max_temperature = Value::U8(u8::MIN);
        // min
        let mut min_start_timestamp = Value::Time(u32::MAX);
        let mut min_altitude = Value::U16(u16::MAX);
        let mut min_heart_rate = Value::U8(u8::MAX);
        // sum
        let mut total_elapsed_time = Value::U32(0_u32);
        let mut total_timer_time = Value::U32(0_u32);
        let mut total_distance = Value::U32(0_u32);
        let mut total_moving_time = Value::U32(0_u32);
        let mut total_calories = Value::U16(0_u16);
        let mut total_ascent = Value::U16(0_u16);
        let mut total_descent = Value::U16(0_u16);
        // avg
        let mut avg_speed = Value::I32(0_i32);
        let mut avg_speed_count = 0_i32;
        let mut avg_power = Value::I32(0_i32);
        let mut avg_power_count = 0_i32;
        let mut normal_power = Value::I32(0_i32);
        let mut normal_power_count = 0_i32;
        let mut avg_altitude = Value::I32(0_i32);
        let mut avg_altitude_count = 0_i32;
        let mut avg_grade = Value::I32(0_i32);
        let mut avg_grade_count = 0_i32;
        let mut avg_pos_grade = Value::I32(0_i32);
        let mut avg_pos_grade_count = 0_i32;
        let mut avg_neg_grade = Value::I32(0_i32);
        let mut avg_neg_grade_count = 0_i32;
        let mut avg_pos_vertical_speed = Value::I32(0_i32);
        let mut avg_pos_vertical_speed_count = 0_i32;
        let mut avg_neg_vertical_speed = Value::I32(0_i32);
        let mut avg_neg_vertical_speed_count = 0_i32;
        let mut avg_heart_rate = Value::I32(0_i32);
        let mut avg_heart_rate_count = 0_i32;
        let mut avg_cadence = Value::I32(0_i32);
        let mut avg_cadence_count = 0_i32;
        let mut avg_temperature = Value::I32(0_i32);
        let mut avg_temperature_count = 0_i32;

        for session in sessions {
            merge_stats!(
                // max
                max 253, max_stop_timestamp, session,
                max 15, max_speed, session,
                max 21, max_power, session,
                max 50, max_altitude, session,
                max 55, max_pos_grade, session,
                max 56, max_neg_grade, session,
                max 17, max_heart_rate, session,
                max 19, max_cadence, session,
                max 58, max_temperature, session,
                // min
                min 2, min_start_timestamp, session,
                min 71, min_altitude, session,
                min 64, min_heart_rate, session,
                // sum
                sum 7, total_elapsed_time, session,
                sum 8, total_timer_time, session,
                sum 9, total_distance, session,
                sum 59, total_moving_time, session,
                sum 11, total_calories, session,
                sum 22, total_ascent, session,
                sum 23, total_descent, session,
                // avg
                avg 14, avg_speed, avg_speed_count, session,
                avg 20, avg_power, avg_power_count, session,
                avg 34, normal_power, normal_power_count, session,
                avg 49, avg_altitude, avg_altitude_count, session,
                avg 52, avg_grade, avg_grade_count, session,
                avg 53, avg_pos_grade, avg_pos_grade_count, session,
                avg 54, avg_neg_grade, avg_neg_grade_count, session,
                avg 60, avg_pos_vertical_speed, avg_pos_vertical_speed_count, session,
                avg 61, avg_neg_vertical_speed, avg_neg_vertical_speed_count, session,
                avg 16, avg_heart_rate, avg_heart_rate_count, session,
                avg 18, avg_cadence, avg_cadence_count, session,
                avg 57, avg_temperature, avg_temperature_count, session,
            );
        }

        // Update merged session fields
        // max
        update_field!(merged_session.data.values, 253, max_stop_timestamp);
        update_field!(merged_session.data.values, 15, max_speed);
        update_field!(merged_session.data.values, 21, max_power);
        update_field!(merged_session.data.values, 50, max_altitude);
        update_field!(merged_session.data.values, 55, max_pos_grade);
        update_field!(merged_session.data.values, 56, max_neg_grade);
        update_field!(merged_session.data.values, 17, max_heart_rate);
        update_field!(merged_session.data.values, 19, max_cadence);
        update_field!(merged_session.data.values, 58, max_temperature);
        // min
        update_field!(merged_session.data.values, 2, min_start_timestamp);
        update_field!(merged_session.data.values, 71, min_altitude);
        update_field!(merged_session.data.values, 64, min_heart_rate);
        // sum
        update_field!(merged_session.data.values, 7, total_elapsed_time);
        update_field!(merged_session.data.values, 8, total_timer_time);
        update_field!(merged_session.data.values, 9, total_distance);
        update_field!(merged_session.data.values, 59, total_moving_time);
        update_field!(merged_session.data.values, 11, total_calories);
        update_field!(merged_session.data.values, 22, total_ascent);
        update_field!(merged_session.data.values, 23, total_descent);
        // avg
        if avg_speed_count > 0 {
            let avg_speed = <Value as Into<i32>>::into(avg_speed).div(avg_speed_count);
            update_field!(merged_session.data.values, 14, Value::U16(avg_speed as u16));
        }
        if avg_power_count > 0 {
            let avg_power = <Value as Into<i32>>::into(avg_power).div(avg_power_count);
            update_field!(merged_session.data.values, 20, Value::U16(avg_power as u16));
        }
        if normal_power_count > 0 {
            let normal_power = <Value as Into<i32>>::into(normal_power).div(normal_power_count);
            update_field!(
                merged_session.data.values,
                34,
                Value::U16(normal_power as u16)
            );
        }
        if avg_altitude_count > 0 {
            let avg_altitude = <Value as Into<i32>>::into(avg_altitude).div(avg_altitude_count);
            update_field!(
                merged_session.data.values,
                49,
                Value::U16(avg_altitude as u16)
            );
        }
        if avg_grade_count > 0 {
            let avg_grade = <Value as Into<i32>>::into(avg_grade).div(avg_grade_count);
            update_field!(merged_session.data.values, 52, Value::I16(avg_grade as i16));
        }
        if avg_pos_grade_count > 0 {
            let avg_pos_grade = <Value as Into<i32>>::into(avg_pos_grade).div(avg_pos_grade_count);
            update_field!(
                merged_session.data.values,
                53,
                Value::I16(avg_pos_grade as i16)
            );
        }
        if avg_neg_grade_count > 0 {
            let avg_neg_grade = <Value as Into<i32>>::into(avg_neg_grade).div(avg_neg_grade_count);
            update_field!(
                merged_session.data.values,
                54,
                Value::I16(avg_neg_grade as i16)
            );
        }
        if avg_pos_vertical_speed_count > 0 {
            let avg_pos_vertical_speed = <Value as Into<i32>>::into(avg_pos_vertical_speed)
                .div(avg_pos_vertical_speed_count);
            update_field!(
                merged_session.data.values,
                60,
                Value::I16(avg_pos_vertical_speed as i16)
            );
        }
        if avg_neg_vertical_speed_count > 0 {
            let avg_neg_vertical_speed = <Value as Into<i32>>::into(avg_neg_vertical_speed)
                .div(avg_neg_vertical_speed_count);
            update_field!(
                merged_session.data.values,
                61,
                Value::I16(avg_neg_vertical_speed as i16)
            );
        }
        if avg_heart_rate_count > 0 {
            let avg_heart_rate =
                <Value as Into<i32>>::into(avg_heart_rate).div(avg_heart_rate_count);
            update_field!(
                merged_session.data.values,
                16,
                Value::U8(avg_heart_rate as u8)
            );
        }
        if avg_cadence_count > 0 {
            let avg_cadence = <Value as Into<i32>>::into(avg_cadence).div(avg_cadence_count);
            update_field!(merged_session.data.values, 18, Value::U8(avg_cadence as u8));
        }
        if avg_temperature_count > 0 {
            let avg_temperature =
                <Value as Into<i32>>::into(avg_temperature).div(avg_temperature_count);
            update_field!(
                merged_session.data.values,
                57,
                Value::I8(avg_temperature as i8)
            );
        }

        Some(merged_session)
    }

    pub fn get_session(&self) -> Option<(usize, FitDataMessage)> {
        for (index, message) in self.data.iter().enumerate() {
            match message {
                FitMessage::Definition(_) => {}
                FitMessage::Data(msg) => match msg.data.message_type {
                    MessageType::Session => {
                        return Some((index, msg.clone()));
                    }
                    _ => {}
                },
            };
        }
        None
    }
}
