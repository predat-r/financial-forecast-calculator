use polars::frame::DataFrame;
use polars::prelude::{CsvWriter, PolarsResult, SerWriter};
use std::fs::File;
use std::path::PathBuf;

pub fn export_to_csv(mut df: DataFrame, path: PathBuf) -> PolarsResult<()> {
    let file = File::create(path)?;
    CsvWriter::new(file).finish(&mut df)
}
