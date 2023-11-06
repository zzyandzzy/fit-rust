use binrw::__private::Required;
use binrw::{BinResult, BinWrite, BinWriterExt, Endian};
use std::io::{Read, Seek, SeekFrom, Write};

pub fn skip_bytes<R>(map: &mut R, s: u8)
where
    R: Seek,
{
    map.seek(SeekFrom::Current(s.into())).unwrap();
}

pub fn read_u8<R>(map: &mut R) -> u8
where
    R: Read,
{
    let mut buf: [u8; 1] = [0];
    let _ = map.read(&mut buf);
    buf[0]
}

pub fn read_i8<R>(map: &mut R) -> i8
where
    R: Read,
{
    read_u8(map) as i8
}
pub fn read_u16<R>(map: &mut R, endian: Endian) -> u16
where
    R: Read,
{
    let arr = arr2(map);
    if endian == Endian::Little {
        u16::from_le_bytes(arr)
    } else {
        u16::from_be_bytes(arr)
    }
}

pub fn read_i16<R>(map: &mut R, endian: Endian) -> i16
where
    R: Read,
{
    let arr = arr2(map);
    if endian == Endian::Little {
        i16::from_le_bytes(arr)
    } else {
        i16::from_be_bytes(arr)
    }
}

pub fn write_bin<R, T>(map: &mut R, b: T, endian: Endian) -> BinResult<()>
where
    R: Write + Seek,
    T: BinWrite,
    for<'a> T::Args<'a>: Required,
{
    if endian == Endian::Little {
        map.write_le(&b)
    } else {
        map.write_be(&b)
    }
}

pub fn read_i32<R>(map: &mut R, endian: Endian) -> i32
where
    R: Read,
{
    let arr = arr4(map);
    if endian == Endian::Little {
        i32::from_le_bytes(arr)
    } else {
        i32::from_be_bytes(arr)
    }
}

pub fn read_u32<R>(map: &mut R, endian: Endian) -> u32
where
    R: Read,
{
    let arr = arr4(map);
    if endian == Endian::Little {
        u32::from_le_bytes(arr)
    } else {
        u32::from_be_bytes(arr)
    }
}
pub fn read_u64<R>(map: &mut R, endian: Endian) -> u64
where
    R: Read,
{
    let arr = arr8(map);
    if endian == Endian::Little {
        u64::from_le_bytes(arr)
    } else {
        u64::from_be_bytes(arr)
    }
}

pub fn read_i64<R>(map: &mut R, endian: Endian) -> i64
where
    R: Read,
{
    let arr = arr8(map);
    if endian == Endian::Little {
        i64::from_le_bytes(arr)
    } else {
        i64::from_be_bytes(arr)
    }
}

fn arr2<R>(map: &mut R) -> [u8; 2]
where
    R: Read,
{
    let mut buf: [u8; 2] = [0; 2];
    let _ = map.read(&mut buf);
    buf
}

pub fn arr4<R>(map: &mut R) -> [u8; 4]
where
    R: Read,
{
    let mut buf: [u8; 4] = [0; 4];
    let _ = map.read(&mut buf);
    buf
}

fn arr8<R>(map: &mut R) -> [u8; 8]
where
    R: Read,
{
    let mut buf: [u8; 8] = [0; 8];
    let _ = map.read(&mut buf);
    buf
}
