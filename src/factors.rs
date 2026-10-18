use polars::prelude::{DataType, LazyFrame, NULL, col, lit, when};

pub fn roe(lf: LazyFrame) -> LazyFrame {
    lf.with_columns([
        when(col("equity").is_null().or(col("equity").lt_eq(lit(0))))
            .then(lit(NULL))
            .otherwise(col("netinc") / col("equity").cast(DataType::Float64) * lit(100))
            .alias("roe"),
    ])
}
