use binrw::BinResult;
use fit_rust::Fit;

fn main() -> BinResult<()> {
    Fit::merge(vec!["test1.fit", "test2.fit", "test3.fit"], "test.fit")?;
    Ok(())
}
