use crate::protocol::consts::{COORD_SEMICIRCLES_CALC, PSEUDO_EPOCH};
use crate::protocol::field_type_enum::FieldType;
use crate::protocol::get_field_offset::get_field_offset_fn;
use crate::protocol::get_field_scale::get_field_scale_fn;
use crate::protocol::get_field_string_value::get_field_string_value_fn;
use crate::protocol::get_field_type::get_field_type_fn;
use crate::protocol::io::{
    read_i16, read_i32, read_i64, read_i8, read_u16, read_u32, read_u64, read_u8, skip_bytes,
    write_bin,
};
use crate::protocol::message_type::MessageType;
use crate::protocol::value::Value;
use crate::protocol::{
    DefinitionMessage, FieldDefinition, MatchFieldTypeFn, MatchOffsetFn, MatchScaleFn,
};
use binrw::{BinResult, Endian};
use copyless::VecHelper;
use std::fmt::{Debug, Formatter};
use std::io::{Read, Seek, Write};
use tracing::warn;

#[derive(Clone, PartialEq)]
pub struct DataField {
    pub field_num: u8,

    pub value: Option<Value>,
}

impl Debug for DataField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            None => {
                write!(f, " {{ t: {:?}, v: None }}", &self.field_num)
            }
            Some(v) => {
                write!(f, " {{ t: {:?}, v: {:?} }}", &self.field_num, v)
            }
        }
    }
}

impl DataField {
    pub fn new(fnum: u8, v: Option<Value>) -> Self {
        Self {
            field_num: fnum,
            value: v,
        }
    }

    #[binrw::parser(reader, endian)]
    pub fn parse_data_field(
        message_type: MessageType,
        fields: &Vec<FieldDefinition>,
    ) -> BinResult<Vec<DataField>> {
        let mut values = Vec::with_capacity(fields.len());
        if message_type == MessageType::None {
            let size_sum: u8 = fields.iter().map(|field| field.size).sum();
            warn!(
                "message_type == MessageType::None, skip_bytes: {}",
                size_sum
            );
            skip_bytes(reader, size_sum);
        } else {
            for fd in fields.iter() {
                let data = DataField::read_next_field(fd.size, fd.base_type, reader, endian);
                values
                    .alloc()
                    .init(DataField::new(fd.definition_number, data));
            }
            // check each value in case the raw value needs further processing
            let scales = get_field_scale_fn(message_type);
            let offsets = get_field_offset_fn(message_type);
            let fields = get_field_type_fn(message_type);
            for v in &mut values {
                DataField::process_read_value(v, fields, scales, offsets);
            }
        }
        // values.shrink_to_fit();
        Ok(values)
    }

    #[allow(clippy::cognitive_complexity)]
    pub fn read_next_field<R>(
        size: u8,
        base_type: u8,
        reader: &mut R,
        endian: Endian,
    ) -> Option<Value>
    where
        R: Read + Seek,
    {
        match base_type {
            0 | 13 => {
                // enum / byte
                if size > 1 {
                    skip_bytes(reader, size);
                    None
                } else {
                    match read_u8(reader) {
                        v => Some(Value::U8(v)),
                    }
                }
            }
            1 => {
                // sint8
                if size > 1 {
                    skip_bytes(reader, size);
                    None
                } else {
                    match read_i8(reader) {
                        v => Some(Value::I8(v)),
                    }
                }
            }
            2 => {
                // uint8
                if size > 1 {
                    let mut buf: Vec<_> = Vec::with_capacity(size.into());
                    let _ = reader.take(size.into()).read_to_end(&mut buf);
                    Some(Value::ArrU8(buf))
                } else {
                    match read_u8(reader) {
                        v => Some(Value::U8(v)),
                    }
                }
            }
            3 => {
                // sint16
                let number_of_values = size / 2;
                if number_of_values > 1 {
                    skip_bytes(reader, size);
                    None
                } else {
                    let val = read_i16(reader, endian);
                    Some(Value::I16(val))
                }
            }
            4 => {
                // uint16
                let number_of_values = size / 2;
                if number_of_values > 1 {
                    let c: Vec<_> = (0..number_of_values)
                        .filter_map(|_| match read_u16(reader, endian) {
                            0xFFFF => None,
                            v => Some(v),
                        })
                        .collect();
                    if c.is_empty() {
                        None
                    } else {
                        Some(Value::ArrU16(c))
                    }
                } else {
                    let val = read_u16(reader, endian);
                    Some(Value::U16(val))
                }
            }
            5 => {
                // sint32
                let number_of_values = size / 4;
                if number_of_values > 1 {
                    skip_bytes(reader, size);
                    None
                } else {
                    let val = read_i32(reader, endian);
                    Some(Value::I32(val))
                }
            }
            6 => {
                // uint32
                let number_of_values = size / 4;
                if number_of_values > 1 {
                    let c: Vec<_> = (0..number_of_values)
                        .filter_map(|_| match read_u32(reader, endian) {
                            0xFFFF_FFFF => None,
                            v => Some(v),
                        })
                        .collect();
                    if c.is_empty() {
                        None
                    } else {
                        Some(Value::ArrU32(c))
                    }
                } else {
                    let val = read_u32(reader, endian);
                    Some(Value::U32(val))
                }
            }
            7 => {
                // string
                let mut buf: Vec<_> = Vec::with_capacity(size.into());
                let _ = reader.take(size.into()).read_to_end(&mut buf);
                if let Ok(string) = String::from_utf8(buf) {
                    Some(Value::String(string))
                } else {
                    None
                }
            }
            8 => {
                // float32
                let number_of_values = size / 4;
                if number_of_values > 1 {
                    skip_bytes(reader, size);
                    None
                } else {
                    let uval = read_u32(reader, endian);
                    let val = f32::from_bits(uval);
                    Some(Value::F32(val))
                }
            }
            9 => {
                // float64
                let number_of_values = size / 8;
                if number_of_values > 1 {
                    skip_bytes(reader, size);
                    None
                } else {
                    let uval = read_u64(reader, endian);
                    let val = f64::from_bits(uval);
                    Some(Value::F64(val))
                }
            }
            10 => {
                // uint8z
                if size > 1 {
                    skip_bytes(reader, size);
                    None
                } else {
                    let val = read_u8(reader);
                    Some(Value::U8(val))
                }
            }
            11 => {
                // uint16z
                let number_of_values = size / 2;
                if number_of_values > 1 {
                    skip_bytes(reader, size);
                    None
                } else {
                    let val = read_u16(reader, endian);
                    Some(Value::U16(val))
                }
            }
            12 => {
                // uint32z
                let number_of_values = size / 4;
                if number_of_values > 1 {
                    skip_bytes(reader, size);
                    None
                } else {
                    let val = read_u32(reader, endian);
                    Some(Value::U32(val))
                }
            }
            14 => {
                // sint64
                let number_of_values = size / 8;
                if number_of_values > 1 {
                    skip_bytes(reader, size);
                    None
                } else {
                    let val = read_i64(reader, endian);
                    Some(Value::I64(val))
                }
            }
            15 => {
                // uint64
                let number_of_values = size / 8;
                if number_of_values > 1 {
                    skip_bytes(reader, size);
                    None
                } else {
                    let val = read_u64(reader, endian);
                    Some(Value::U64(val))
                }
            }
            16 => {
                // uint64z
                let number_of_values = size / 8;
                if number_of_values > 1 {
                    skip_bytes(reader, size);
                    None
                } else {
                    let val = read_u64(reader, endian);
                    Some(Value::U64(val))
                }
            }
            _ => None,
        }
    }

    #[allow(unused_must_use)]
    fn process_read_value(
        v: &mut DataField,
        fields: MatchFieldTypeFn,
        scales: MatchScaleFn,
        offsets: MatchOffsetFn,
    ) {
        match fields(v.field_num as usize) {
            FieldType::None => (),
            FieldType::Coordinates => {
                if let Some(Value::I32(ref inner)) = v.value {
                    let coord = *inner as f32 * COORD_SEMICIRCLES_CALC;
                    std::mem::replace(&mut v.value, Some(Value::F32(coord)));
                }
            }
            FieldType::Timestamp | FieldType::DateTime => {
                if let Some(Value::U32(ref inner)) = v.value {
                    let date = *inner + PSEUDO_EPOCH;
                    std::mem::replace(&mut v.value, Some(Value::Time(date)));
                }
            }
            FieldType::LocalDateTime => {
                if let Some(Value::U32(ref inner)) = v.value {
                    let time = *inner + PSEUDO_EPOCH - 3600;
                    std::mem::replace(&mut v.value, Some(Value::Time(time)));
                }
            }
            FieldType::String | FieldType::LocaltimeIntoDay => {}
            FieldType::Uint8
            | FieldType::Uint8Z
            | FieldType::Uint16
            | FieldType::Uint16Z
            | FieldType::Uint32
            | FieldType::Uint32Z
            | FieldType::Sint8 => {
                if let Some(s) = scales(v.field_num as usize) {
                    match &mut v.value {
                        None => {}
                        Some(val) => {
                            val.scale(s);
                        }
                    }
                }
                if let Some(o) = offsets(v.field_num as usize) {
                    match &mut v.value {
                        None => {}
                        Some(val) => {
                            val.offset(o);
                        }
                    }
                }
            }
            f => {
                if let Some(Value::U8(k)) = v.value {
                    if let Some(t) = get_field_string_value_fn(f, usize::from(k)) {
                        std::mem::replace(&mut v.value, Some(Value::Enum(t)));
                    }
                } else if let Some(Value::U16(k)) = v.value {
                    if let Some(t) = get_field_string_value_fn(f, usize::from(k)) {
                        std::mem::replace(&mut v.value, Some(Value::Enum(t)));
                    }
                }
            }
        }
    }

    fn process_write_value(
        v: &DataField,
        fields: MatchFieldTypeFn,
        scales: MatchScaleFn,
        offsets: MatchOffsetFn,
    ) -> Option<Value> {
        match fields(v.field_num as usize) {
            FieldType::None => None,
            FieldType::Coordinates => {
                if let Some(Value::F32(ref inner)) = v.value {
                    let coord = *inner / COORD_SEMICIRCLES_CALC;
                    return Some(Value::I32(coord as i32));
                }
                None
            }
            FieldType::DateTime | FieldType::Timestamp => {
                if let Some(Value::Time(ref inner)) = v.value {
                    let date = *inner - PSEUDO_EPOCH;
                    return Some(Value::U32(date));
                }
                None
            }
            FieldType::LocalDateTime => {
                if let Some(Value::Time(ref inner)) = v.value {
                    let date = *inner - PSEUDO_EPOCH + 3600;
                    return Some(Value::U32(date));
                }
                None
            }
            FieldType::String | FieldType::LocaltimeIntoDay => v.clone().value,
            FieldType::Uint8
            | FieldType::Uint8Z
            | FieldType::Uint16
            | FieldType::Uint16Z
            | FieldType::Uint32
            | FieldType::Uint32Z
            | FieldType::Sint8 => {
                let mut v = v.clone();
                if let Some(s) = scales(v.field_num as usize) {
                    match &mut v.value {
                        None => {}
                        Some(val) => {
                            val.rescale(s);
                        }
                    }
                }
                if let Some(o) = offsets(v.field_num as usize) {
                    match &mut v.value {
                        None => {}
                        Some(val) => {
                            val.reoffset(o);
                        }
                    }
                }
                v.value
            }
            _ => {
                let v = v.clone();
                // if let Some(Value::U8(k)) = v.value {
                //     if let Some(t) = get_field_string_value_fn(f, usize::from(k)) {
                //         std::mem::replace(&mut v.value, Some(Value::Enum(t)));
                //     }
                // } else if let Some(Value::U16(k)) = v.value {
                //     if let Some(t) = get_field_string_value_fn(f, usize::from(k)) {
                //         std::mem::replace(&mut v.value, Some(Value::Enum(t)));
                //     }
                // }
                v.value
            }
        }
    }

    #[binrw::writer(writer, endian)]
    pub fn write_data_field(
        values: &Vec<DataField>,
        message_type: MessageType,
        def_msg: &DefinitionMessage,
    ) -> BinResult<()> {
        let scales = get_field_scale_fn(message_type);
        let offsets = get_field_offset_fn(message_type);
        let fields = get_field_type_fn(message_type);
        for (i, field) in values.iter().enumerate() {
            let value = DataField::process_write_value(field, fields, scales, offsets);
            match value {
                None => {
                    let def_field = def_msg.fields.get(i);
                    DataField::write_none(writer, endian, def_field);
                }
                Some(v) => {
                    let _ = match &v {
                        Value::U8(v) => write_bin(writer, v, endian),
                        Value::I8(v) => write_bin(writer, v, endian),
                        Value::U16(v) => write_bin(writer, v, endian),
                        Value::I16(v) => write_bin(writer, v, endian),
                        Value::U32(v) => write_bin(writer, v, endian),
                        Value::I32(v) => write_bin(writer, v, endian),
                        Value::F32(v) => write_bin(writer, v, endian),
                        Value::F64(v) => write_bin(writer, v, endian),
                        Value::ArrU8(v) => write_bin(writer, v, endian),
                        Value::ArrU16(v) => write_bin(writer, v, endian),
                        Value::ArrU32(v) => write_bin(writer, v, endian),
                        Value::Time(v) => write_bin(writer, v, endian),
                        Value::String(v) => write_bin(writer, v.as_bytes(), endian),
                        Value::Enum(_) => {
                            let def_field = def_msg.fields.get(i);
                            Ok(DataField::write_none(writer, endian, def_field))
                        }
                        _ => Ok(()),
                    };
                }
            };
        }
        Ok(())
    }

    fn write_none<W>(writer: &mut W, endian: Endian, def_field: Option<&FieldDefinition>)
    where
        W: Write + Seek,
    {
        match def_field {
            None => {
                warn!("Can not write from None!");
            }
            Some(def_field) => {
                let vec: Vec<u8> = vec![0; def_field.size as usize];
                let _ = write_bin(writer, vec, endian);
            }
        }
    }
}
