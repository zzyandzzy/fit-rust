mod consts;
mod data_field;
pub mod field_type_enum;
mod get_field_offset;
mod get_field_scale;
mod get_field_string_value;
mod get_field_type;
mod io;
pub mod message_type;
mod value;

use crate::protocol::consts::{
    COMPRESSED_HEADER_LOCAL_MESSAGE_NUMBER_MASK, COMPRESSED_HEADER_MASK,
    COMPRESSED_HEADER_TIME_OFFSET_MASK, DEFINITION_HEADER_MASK, DEVELOPER_FIELDS_MASK,
    FIELD_DEFINITION_BASE_NUMBER, LOCAL_MESSAGE_NUMBER_MASK,
};
use crate::protocol::data_field::DataField;
use crate::protocol::field_type_enum::FieldType;
use crate::protocol::message_type::MessageType;
use binrw::{binrw, BinReaderExt, BinResult, BinWrite, Endian};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs::write;
use std::io::Cursor;
use std::path::Path;
use tracing::debug;

pub type MatchScaleFn = fn(usize) -> Option<f32>;
pub type MatchOffsetFn = fn(usize) -> Option<i16>;
pub type MatchFieldTypeFn = fn(usize) -> FieldType;

pub struct Fit {
    pub header: FitHeader,

    pub data: Vec<FitDataMessage>,

    map: HashMap<u8, VecDeque<FitDefinitionMessage>>,
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

        let mut data: Vec<FitDataMessage> = Vec::new();
        loop {
            let message_header: FitMessageHeader = cursor.read_ne()?;
            match message_header.definition {
                true => {
                    let definition_message: DefinitionMessage =
                        cursor.read_ne_args((message_header.dev_fields,))?;
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
                    debug!("definition: {:?}", definition);
                    let data_message: DataMessage = cursor.read_ne_args((definition,))?;
                    debug!("massage: {:?}", data_message);
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
        Ok(Fit { header, data, map })
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> BinResult<()> {
        let mut buf = Vec::new();
        self.write_buf(&mut buf)?;
        write(path, &buf)?;
        Ok(())
    }

    pub fn write_buf(&self, buf: &mut Vec<u8>) -> BinResult<()> {
        let mut map = self.map.clone();

        let mut writer = Cursor::new(buf);
        self.header.write(&mut writer)?;

        for item in &self.data {
            let mut definition_message = match map.get_mut(&item.header.local_num) {
                None => continue,
                Some(mut queue) => queue.pop_back(),
            };
            match definition_message {
                None => {}
                Some(def) => {
                    def.write(&mut writer)?;
                }
            }

            item.write(&mut writer)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
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

impl FitHeader {
    fn new(data_size: u32) -> Self {
        Self {
            header_size: 14,
            protocol_version: 0x10,
            profile_version: 0x5408,
            data_size,
            data_type: ".FIT".to_string(),
            crc: Some(0x0000),
        }
    }
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

#[derive(BinWrite, Debug, Clone, PartialEq)]
#[bw(little)]
pub struct FitDataMessage {
    pub header: FitMessageHeader,
    pub message: DataMessage,
}

#[derive(Clone, Debug, PartialEq)]
#[binrw]
#[br(import(definition: &FitDefinitionMessage))]
#[bw(little)]
pub struct DataMessage {
    #[br(parse_with = message_type::parse_message_type, args(definition.message.global_message_number))]
    #[bw(ignore)]
    pub message_type: MessageType,

    #[br(parse_with = DataField::parse_data_field, args(message_type, &definition.message.fields), is_little = (definition.message.endian == Endian::Little))]
    #[bw(write_with = DataField::write_data_field, args(message_type.clone()))]
    pub values: Vec<DataField>,
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
        let mut x = self.local_num;
        if self.compressed_header {
            x |= COMPRESSED_HEADER_MASK;
        }
        if self.definition {
            x |= DEFINITION_HEADER_MASK;
        }
        if self.dev_fields {
            x |= DEVELOPER_FIELDS_MASK;
        }
        x
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

#[cfg(test)]
mod tests {

    const CRC_TABLE: [u16; 16] = [
        0x0000, 0xCC01, 0xD801, 0x1400, 0xF001, 0x3C00, 0x2800, 0xE401, 0xA001, 0x6C00, 0x7800,
        0xB401, 0x5000, 0x9C01, 0x8801, 0x4400,
    ];

    fn fit_crc_get16(crc: u16, byte: u8) -> u16 {
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

    #[test]
    fn fit_crc_get16_test() {
        let crc = fit_crc_get16(0x0587, 0x2);
        println!("CRC: 0x{:04X}", crc);
    }
}
