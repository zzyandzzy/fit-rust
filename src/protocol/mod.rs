mod consts;
pub mod data_field;
mod get_field_offset;
mod get_field_scale;
mod get_field_string_value;
mod get_field_type;
pub mod io;
pub mod macros;
pub mod message_type;
pub mod value;

use crate::protocol::consts::{
    COMPRESSED_HEADER_LOCAL_MESSAGE_NUMBER_MASK, COMPRESSED_HEADER_MASK,
    COMPRESSED_HEADER_TIME_OFFSET_MASK, CRC_TABLE, DEFINITION_HEADER_MASK, DEVELOPER_FIELDS_MASK,
    FIELD_DEFINITION_BASE_ENDIAN, FIELD_DEFINITION_BASE_NUMBER, LOCAL_MESSAGE_NUMBER_MASK,
};
use crate::protocol::data_field::DataField;
use crate::protocol::get_field_string_value::FieldType;
use crate::protocol::message_type::MessageType;
use binrw::{binrw, BinRead, BinResult, BinWrite, Endian};
use std::fmt::Debug;
use std::io::{Seek, Write};

pub type MatchScaleFn = fn(usize) -> Option<f32>;
pub type MatchOffsetFn = fn(usize) -> Option<i16>;
pub type MatchFieldTypeFn = fn(usize) -> FieldType;

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
    pub fn write<W>(&self, writer: &mut W, def_msg: &DefinitionMessage) -> BinResult<()>
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

    pub base_type: FieldDefBaseType,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[binrw]
#[br(map = FieldDefBaseType::from_bytes)]
#[bw(map = FieldDefBaseType::to_bytes)]
pub struct FieldDefBaseType {
    pub val: u8,

    pub endian: Endian,
}

impl FieldDefBaseType {
    fn to_bytes(&self) -> u8 {
        if self.endian == Endian::Little {
            self.val
        } else {
            self.val | FIELD_DEFINITION_BASE_ENDIAN
        }
    }

    fn from_bytes(x: u8) -> FieldDefBaseType {
        let mut endian = Endian::Little;
        if x & FIELD_DEFINITION_BASE_ENDIAN == FIELD_DEFINITION_BASE_ENDIAN {
            endian = Endian::Big;
        }
        Self {
            val: x & FIELD_DEFINITION_BASE_NUMBER,
            endian,
        }
    }
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
}
