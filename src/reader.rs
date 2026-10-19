use polars::prelude::{LazyCsvReader, LazyFileListReader, LazyFrame, PlRefPath, PolarsResult};

/// Lazily reads financial statement records from a CSV file.
///
/// The returned frame is not collected until a later query is executed. The CSV
/// should contain the columns required by the selected factor calculations and
/// analyses, such as `ticker`, `calendardate`, and financial statement values.
pub fn read_from_csv(path: PlRefPath) -> PolarsResult<LazyFrame> {
    LazyCsvReader::new(path).finish()
}
