use polars::prelude::{LazyCsvReader, LazyFileListReader, LazyFrame, PlRefPath, PolarsResult};

pub fn read_from_csv(path: PlRefPath) -> PolarsResult<LazyFrame> {
    LazyCsvReader::new(path).finish()
}
