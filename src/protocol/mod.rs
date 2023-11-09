mod consts;
pub mod data_field;
mod get_field_offset;
mod get_field_scale;
mod get_field_string_value;
mod get_field_type;
mod io;
pub mod message_type;
pub mod value;

use crate::protocol::consts::{
    COMPRESSED_HEADER_LOCAL_MESSAGE_NUMBER_MASK, COMPRESSED_HEADER_MASK,
    COMPRESSED_HEADER_TIME_OFFSET_MASK, CRC_TABLE, DEFINITION_HEADER_MASK, DEVELOPER_FIELDS_MASK,
    FIELD_DEFINITION_BASE_NUMBER, LOCAL_MESSAGE_NUMBER_MASK,
};
use crate::protocol::data_field::DataField;
use crate::protocol::get_field_string_value::FieldType;
use crate::protocol::io::{skip_bytes, write_bin};
use crate::protocol::message_type::MessageType;
use binrw::{binrw, BinRead, BinReaderExt, BinResult, BinWrite, Endian, Error};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs::{read, write};
use std::io::{Cursor, Seek, SeekFrom, Write};
use std::path::Path;
use tracing::{debug, error};

pub type MatchScaleFn = fn(usize) -> Option<f32>;
pub type MatchOffsetFn = fn(usize) -> Option<i16>;
pub type MatchFieldTypeFn = fn(usize) -> FieldType;

pub struct Fit {
    pub header: FitHeader,

    pub data: Vec<FitDataMessage>,

    map: HashMap<u8, VecDeque<FitDefinitionMessage>>,

    global_def_map: HashMap<u16, DefinitionMessage>,
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
        debug!("header: {:?}", header);
        let mut map: HashMap<u8, VecDeque<FitDefinitionMessage>> = HashMap::new();
        let mut global_def_map: HashMap<u16, DefinitionMessage> = HashMap::new();

        let mut data: Vec<FitDataMessage> = Vec::new();
        loop {
            let message_header: FitMessageHeader = cursor.read_ne()?;
            match message_header.definition {
                true => {
                    if message_header.dev_fields {
                        unimplemented!("message_header.dev_fields is unimplemented");
                    }
                    let definition_message: DefinitionMessage =
                        cursor.read_ne_args((message_header.dev_fields,))?;
                    global_def_map.insert(
                        definition_message.global_message_number,
                        definition_message.clone(),
                    );
                    map.entry(message_header.local_num)
                        .or_insert_with(VecDeque::new)
                        .push_front(FitDefinitionMessage {
                            header: message_header,
                            message: definition_message,
                        });
                }
                false => {
                    let definition = match map.get(&message_header.local_num) {
                        None => continue,
                        Some(queue) => queue.front().unwrap(),
                    };
                    let data_message: DataMessage = cursor.read_ne_args((definition,))?;
                    if data_message.message_type == MessageType::None {
                        debug!("message is None, continue");
                        continue;
                    }
                    if data_message.message_type != MessageType::Record {
                        debug!("definition: {:?}", definition);
                        debug!("massage: {:?}", data_message);
                    }
                    data.push(FitDataMessage {
                        header: message_header,
                        message: data_message,
                    });
                    if cursor.position() >= (header.data_size + header.header_size as u32) as u64 {
                        debug!(
                            "decode finish! pos: {}, size: {:?}KB",
                            cursor.position(),
                            (header.data_size + header.header_size as u32 + 2) / 1024
                        );
                        break;
                    }
                }
            }
        }
        Ok(Fit {
            header,
            data,
            map,
            global_def_map,
        })
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
                debug!("header crc: 0x{:X}", crc);
                write_bin(&mut writer, crc, Endian::Little)?;
            }
        }
        debug!("body crc: 0x{:X}", body_crc);
        writer.seek(SeekFrom::End(0))?;
        write_bin(&mut writer, body_crc, Endian::Little)?;
        writer.flush()?;
        Ok(())
    }

    pub(crate) fn write_buf(&self, buf: &mut Vec<u8>) -> BinResult<FitHeader> {
        let mut map = self.map.clone();
        let global_def_map: HashMap<u16, DefinitionMessage> = self.global_def_map.clone();
        let mut writer = Cursor::new(buf);
        skip_bytes(&mut writer, self.header.header_size);
        for item in &self.data {
            let definition_message = match map.get_mut(&item.header.local_num) {
                None => continue,
                Some(queue) => queue.pop_back(),
            };
            match definition_message {
                None => {}
                Some(def) => {
                    def.write(&mut writer)?;
                }
            }
            if &item.message.message_type == &MessageType::None {
                continue;
            }
            let def_msg = global_def_map.get(&item.message.message_type.to_primitive());
            match def_msg {
                None => {
                    error!(
                        "Error definition message is not define! message type: {:?}",
                        item.message.message_type
                    );
                    return Err(Error::Io(binrw::io::Error::new(
                        binrw::io::ErrorKind::UnexpectedEof,
                        "Error definition message is not define!",
                    )));
                }
                Some(def_msg) => {
                    item.header.write(&mut writer)?;
                    item.message.write(&mut writer, def_msg)?;
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

    #[allow(unused)]
    pub fn merge<P: AsRef<Path>>(files: Vec<P>, path: P) -> BinResult<()> {
        if files.is_empty() || files.len() <= 1 {
            error!("Error files is empty: {:?}", files.len());
            return Err(Error::Io(binrw::io::Error::new(
                binrw::io::ErrorKind::UnexpectedEof,
                "Error files is empty!",
            )));
        }
        let file = read(files.get(0).unwrap()).unwrap();
        let mut fit: Fit = Fit::read(file)?;
        for i in 1..=files.len() - 1 {
            let f = files.get(i).unwrap();
            let f = read(f).unwrap();
            let mut tmp = Fit::read(f)?;
            fit.data.append(&mut tmp.data);
        }

        fit.write(path)
    }
}

#[derive(Debug, Clone)]
#[binrw]
#[brw(little)]
pub struct FitHeader {
    /// https://developer.garmin.com/fit/protocol/
    /// Indicates the length of this file header including header size. Minimum size is 12.
    /// This may be increased in future to add additional optional information
    pub header_size: u8,

    /// Protocol version number as provided in SDK
    pub protocol_version: u8,

    /// Profile version number as provided in SDK
    pub profile_version: u16,

    /// Length of the Data Records section in bytesDoes not include Header or CRC
    pub data_size: u32,

    /// ASCII values for “.FIT”. A FIT binary file opened with a text editor will contain a readable “.FIT” in the first line.
    #[br(map = | x: [u8;4] | String::from_utf8_lossy(&x).to_string())]
    #[bw(map = | _ | ".FIT".as_bytes())]
    pub data_type: String,

    /// Contains the value of the CRC (see CRC ) of Bytes 0 through 11, or may be set to 0x0000. This field is optional.
    #[br(if(header_size == 14))]
    pub crc: Option<u16>,
}

#[derive(BinWrite, Debug, Clone, PartialEq)]
#[bw(little)]
pub struct FitDefinitionMessage {
    pub header: FitMessageHeader,
    pub message: DefinitionMessage,
}

#[derive(Clone, Debug, PartialEq)]
#[binrw]
#[br(import(dev_fields: bool))]
#[bw(little)]
pub struct DefinitionMessage {
    pub reserved: u8,

    #[br(map = DefinitionMessage::read_endian)]
    #[bw(map = DefinitionMessage::write_endian)]
    pub endian: Endian,

    #[br(is_little = (endian == Endian::Little))]
    pub global_message_number: u16,

    pub num_fields: u8,

    #[br(count = num_fields)]
    pub fields: Vec<FieldDefinition>,

    #[br(if(dev_fields))]
    pub dev_num_fields: Option<u8>,

    #[br(if(dev_fields), count = dev_num_fields.unwrap_or_default())]
    pub dev_fields: Option<Vec<DevFieldDefinition>>,
}

impl DefinitionMessage {
    fn read_endian(x: u8) -> Endian {
        if x == 0x0 {
            return Endian::Little;
        }
        Endian::Big
    }
    fn write_endian(b: &Endian) -> u8 {
        match b {
            Endian::Big => 0x1,
            Endian::Little => 0x0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FitDataMessage {
    pub header: FitMessageHeader,
    pub message: DataMessage,
}

#[derive(Clone, Debug, PartialEq, BinRead)]
#[br(import(definition: &FitDefinitionMessage))]
pub struct DataMessage {
    #[br(parse_with = message_type::parse_message_type, args(definition.message.global_message_number))]
    pub message_type: MessageType,

    #[br(parse_with = DataField::parse_data_field, args(message_type, &definition.message.fields), is_little = (definition.message.endian == Endian::Little))]
    pub values: Vec<DataField>,
}

impl DataMessage {
    fn write<W>(&self, writer: &mut W, def_msg: &DefinitionMessage) -> BinResult<()>
    where
        W: Write + Seek,
    {
        DataField::write_data_field(
            &self.values,
            writer,
            Endian::Little,
            (self.message_type, def_msg),
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[binrw]
pub struct FieldDefinition {
    pub definition_number: u8,

    pub size: u8,

    #[br(map = | x: u8 | x & FIELD_DEFINITION_BASE_NUMBER)]
    pub base_type: u8,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[binrw]
pub struct DevFieldDefinition {
    pub field_number: u8,
    pub size: u8,
    pub dev_data_index: u8,
}

#[derive(Clone, Debug, PartialEq)]
#[binrw]
#[br(map = FitMessageHeader::from_bytes)]
#[bw(map = FitMessageHeader::to_bytes)]
pub struct FitMessageHeader {
    pub compressed_header: bool,
    pub definition: bool,
    pub dev_fields: bool,
    pub local_num: u8,
    pub time_offset: Option<u8>,
}

impl FitMessageHeader {
    fn to_bytes(&self) -> u8 {
        if self.compressed_header {
            let mut x = self.time_offset.unwrap_or_default();
            x |= self.local_num << 5;
            x |= COMPRESSED_HEADER_MASK;
            x
        } else {
            let mut x = self.local_num;
            if self.definition {
                x |= DEFINITION_HEADER_MASK;
            }
            if self.dev_fields {
                x |= DEVELOPER_FIELDS_MASK;
            }
            x
        }
    }

    fn from_bytes(x: u8) -> FitMessageHeader {
        if (x & COMPRESSED_HEADER_MASK) == COMPRESSED_HEADER_MASK {
            Self {
                compressed_header: true,
                definition: false,
                dev_fields: false,
                local_num: (x & COMPRESSED_HEADER_LOCAL_MESSAGE_NUMBER_MASK) >> 5,
                time_offset: Some(x & COMPRESSED_HEADER_TIME_OFFSET_MASK),
            }
        } else {
            Self {
                compressed_header: false,
                definition: x & DEFINITION_HEADER_MASK == DEFINITION_HEADER_MASK,
                dev_fields: x & DEVELOPER_FIELDS_MASK == DEVELOPER_FIELDS_MASK,
                local_num: x & LOCAL_MESSAGE_NUMBER_MASK,
                time_offset: None,
            }
        }
    }
}

pub(crate) fn fit_crc_get16(crc: u16, byte: u8) -> u16 {
    // Compute checksum of lower four bits of byte
    let mut tmp = CRC_TABLE[(crc & 0xF) as usize];
    let mut crc = (crc >> 4) & 0x0FFF;
    crc = crc ^ tmp ^ CRC_TABLE[(byte & 0xF) as usize];

    // Now compute checksum of upper four bits of byte
    tmp = CRC_TABLE[(crc & 0xF) as usize];
    crc = (crc >> 4) & 0x0FFF;
    crc = crc ^ tmp ^ CRC_TABLE[((byte >> 4) & 0xF) as usize];

    crc
}

pub(crate) fn calculate_fit_crc(header: &[u8]) -> u16 {
    let mut crc: u16 = 0;
    for &byte in header {
        crc = fit_crc_get16(crc, byte);
    }
    crc
}

#[cfg(test)]
mod tests {
    use crate::protocol::calculate_fit_crc;

    #[test]
    fn fit_crc_get16_test() {
        let example_header: [u8; 12] = [
            0x0E, 0x10, 0x90, 0x05, 0xB3, 0x10, 0x00, 0x00, 0x2E, 0x46, 0x49, 0x54,
        ];
        let header_crc = calculate_fit_crc(&example_header);
        println!("The CRC of the FIT header is: 0x{:X}", header_crc);
        assert_eq!(0x800, header_crc);
    }

    #[test]
    fn fit_crc_test() {
        let buf = std::fs::read("./tests/2015-06-09-21-12-06.fit").unwrap();
        let header = &buf[0..12];
        let header_crc = calculate_fit_crc(&header);
        println!("The CRC of the FIT header is: 0x{:X}", header_crc);
        assert_eq!(0x800, header_crc);
        let end_byte = buf.len() - 2;
        let body = &buf[14..end_byte];
        let body_crc = calculate_fit_crc(&body);
        println!("The CRC of the FIT body is: 0x{:X}", body_crc);
        assert_eq!(0x0CA1, header_crc);
    }
}
