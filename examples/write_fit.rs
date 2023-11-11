use binrw::BinResult;
use fit_rust::Fit;
use std::fs;

fn main() -> BinResult<()> {
    let file = fs::read("test.fit").unwrap();
    let fit: Fit = Fit::read(file)?;
    fit.write("test1.fit")?;
    Ok(())
}
