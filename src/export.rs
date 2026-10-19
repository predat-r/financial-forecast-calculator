use polars::frame::DataFrame;
use polars::prelude::{CsvWriter, PolarsResult, SerWriter};
use std::fs::File;
use std::path::PathBuf;

/// Writes a collected data frame to a CSV file at `path`.
///
/// File creation and serialization failures are returned as Polars errors.
pub fn export_to_csv(mut df: DataFrame, path: PathBuf) -> PolarsResult<()> {
    let file = File::create(path)?;
    CsvWriter::new(file).finish(&mut df)
}
