use binrw::{binrw, io::Cursor, BinReaderExt, BinResult, BinWrite, Endian, NullString};

#[derive(Debug)]
#[binrw]
#[brw(magic = b"DOG")]
struct Dog {
    bone_pile_count: u8,

    #[bw(big)]
    #[br(big, count = bone_pile_count)]
    bone_piles: Vec<u16>,

    #[brw(align_before = 0xA)]
    name: NullString,
}

fn main() -> BinResult<()> {
    let mut reader = Cursor::new(b"DOG\x02\x00\x01\x00\x12\0\0Rudy\0");
    let dog: Dog = reader.read_ne().unwrap();
    assert_eq!(dog.bone_pile_count, 2);
    assert_eq!(dog.bone_piles, &[0x1, 0x12]);
    assert_eq!(dog.name.to_string(), "Rudy");
    println!("{:?}", dog);

    let mut bytes = Vec::new();
    let mut writer = Cursor::new(&mut bytes);

    dog.write_options(&mut writer, Endian::Big, ())?;

    std::fs::write("./tests/dog.bin", &bytes)?;

    Ok(())
}
