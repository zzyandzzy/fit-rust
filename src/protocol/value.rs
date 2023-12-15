//////////
//// Value
//////////

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    Enum(&'static str),
    String(String),
    F32(f32),
    F64(f64),
    I64(i64),
    U64(u64),
    Time(u32),
    ArrU8(Vec<u8>),
    ArrU16(Vec<u16>),
    ArrU32(Vec<u32>),
    None,
}

#[allow(unused_must_use)]
impl Value {
    pub(super) fn rescale(&mut self, val: f32) {
        match self {
            Value::U8(mut inner) => {
                let new_inner = f32::from(inner) * val;
                std::mem::replace(&mut inner, new_inner as u8);
            }
            Value::I8(mut inner) => {
                let new_inner = f32::from(inner) * val;
                std::mem::replace(&mut inner, new_inner as i8);
            }
            Value::U16(mut inner) => {
                let new_inner = f32::from(inner) * val;
                std::mem::replace(&mut inner, new_inner as u16);
            }
            Value::I16(mut inner) => {
                let new_inner = f32::from(inner) * val;
                std::mem::replace(&mut inner, new_inner as i16);
            }
            Value::U32(mut inner) => {
                let new_inner = inner as f32 * val;
                std::mem::replace(&mut inner, new_inner as u32);
            }
            Value::I32(mut inner) => {
                let new_inner = inner as f32 * val;
                std::mem::replace(&mut inner, new_inner as i32);
            }
            _ => (),
        }
    }

    pub(super) fn scale(&mut self, val: f32) {
        match self {
            Value::U8(mut inner) => {
                let new_inner = f32::from(inner) / val;
                std::mem::replace(&mut inner, new_inner as u8);
            }
            Value::I8(mut inner) => {
                let new_inner = f32::from(inner) / val;
                std::mem::replace(&mut inner, new_inner as i8);
            }
            Value::U16(mut inner) => {
                let new_inner = f32::from(inner) / val;
                std::mem::replace(&mut inner, new_inner as u16);
            }
            Value::I16(mut inner) => {
                let new_inner = f32::from(inner) / val;
                std::mem::replace(&mut inner, new_inner as i16);
            }
            Value::U32(mut inner) => {
                let new_inner = inner as f32 / val;
                std::mem::replace(&mut inner, new_inner as u32);
            }
            Value::I32(mut inner) => {
                let new_inner = inner as f32 / val;
                std::mem::replace(&mut inner, new_inner as i32);
            }
            _ => (),
        }
    }
    pub(super) fn offset(&mut self, val: i16) {
        match self {
            Value::U8(mut inner) => {
                let new_inner = i16::from(inner) - val;
                std::mem::replace(&mut inner, new_inner as u8);
            }
            Value::I8(mut inner) => {
                let new_inner = i16::from(inner) - val;
                std::mem::replace(&mut inner, new_inner as i8);
            }
            Value::U16(mut inner) => {
                let new_inner = inner as i16 - val;
                std::mem::replace(&mut inner, new_inner as u16);
            }
            Value::I16(mut inner) => {
                let new_inner = inner - val;
                std::mem::replace(&mut inner, new_inner);
            }
            Value::U32(mut inner) => {
                let new_inner = inner as i16 - val;
                std::mem::replace(&mut inner, new_inner as u32);
            }
            Value::I32(mut inner) => {
                let new_inner = inner as i16 - val;
                std::mem::replace(&mut inner, i32::from(new_inner));
            }
            _ => (),
        }
    }

    pub(super) fn reoffset(&mut self, val: i16) {
        match self {
            Value::U8(mut inner) => {
                let new_inner = i16::from(inner) + val;
                std::mem::replace(&mut inner, new_inner as u8);
            }
            Value::I8(mut inner) => {
                let new_inner = i16::from(inner) + val;
                std::mem::replace(&mut inner, new_inner as i8);
            }
            Value::U16(mut inner) => {
                let new_inner = inner as i16 + val;
                std::mem::replace(&mut inner, new_inner as u16);
            }
            Value::I16(mut inner) => {
                let new_inner = inner + val;
                std::mem::replace(&mut inner, new_inner);
            }
            Value::U32(mut inner) => {
                let new_inner = inner as i16 + val;
                std::mem::replace(&mut inner, new_inner as u32);
            }
            Value::I32(mut inner) => {
                let new_inner = inner as i16 + val;
                std::mem::replace(&mut inner, i32::from(new_inner));
            }
            _ => (),
        }
    }
}

impl From<Value> for i8 {
    fn from(item: Value) -> Self {
        match item {
            Value::I8(v) => v,
            _ => panic!("can't call this on a non-i8 variant"),
        }
    }
}

impl From<Value> for u8 {
    fn from(item: Value) -> Self {
        match item {
            Value::U8(v) => v,
            _ => panic!("can't call this on a non-u8 variant"),
        }
    }
}

impl From<Value> for i16 {
    fn from(item: Value) -> Self {
        match item {
            Value::I16(v) => v,
            _ => panic!("can't call this on a non-u16 variant"),
        }
    }
}

impl From<Value> for u16 {
    fn from(item: Value) -> Self {
        match item {
            Value::U16(v) => v,
            _ => panic!("can't call this on a non-u16 variant"),
        }
    }
}

impl From<Value> for i32 {
    fn from(item: Value) -> Self {
        match item {
            Value::I32(v) => v,
            _ => panic!("can't call this on a non-u32 variant"),
        }
    }
}

impl From<Value> for u32 {
    fn from(item: Value) -> Self {
        match item {
            Value::U32(v) => v,
            Value::Time(v) => v,
            _ => panic!("can't call this on a non-u32 variant"),
        }
    }
}

impl From<Value> for f32 {
    fn from(item: Value) -> Self {
        match item {
            Value::F32(v) => v,
            _ => panic!("can't call this on a non-u32 variant"),
        }
    }
}

impl From<Value> for &str {
    fn from(item: Value) -> Self {
        match item {
            Value::Enum(v) => v,
            _ => panic!("can't call this on a non-u32 variant"),
        }
    }
}

impl From<Value> for String {
    fn from(item: Value) -> Self {
        match item {
            Value::Enum(v) => v.into(),
            _ => panic!("can't call this on a non-u32 variant"),
        }
    }
}
