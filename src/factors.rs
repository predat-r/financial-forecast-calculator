use polars::prelude::DataType::Float64;
use polars::prelude::{LazyFrame, NULL, cast, col, lit, when};

pub fn roe(lf: LazyFrame) -> LazyFrame {
    lf.with_columns([when(col("equity").is_null().or(col("equity").eq(lit(0))))
        .then(lit(NULL))
        .otherwise(col("netinc").cast(Float64) / col("equity").cast(Float64) * lit(100))
        .alias("roe")])
}
