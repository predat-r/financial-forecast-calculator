use polars::prelude::{DataType, LazyFrame, NULL, col, lit, when};

pub fn roe(lf: LazyFrame) -> LazyFrame {
    lf.with_columns([
        when(col("equity").is_null().or(col("equity").lt_eq(lit(0))))
            .then(lit(NULL))
            .otherwise(col("netinc") / col("equity").cast(DataType::Float64) * lit(100))
            .alias("roe"),
    ])
}

pub fn operating_margin(lf: LazyFrame) -> LazyFrame {
    lf.with_columns([
        when(col("revenue").is_null().or(col("revenue").lt_eq(lit(0))))
            .then(lit(NULL))
            .otherwise(col("opinc") / col("revenue").cast(DataType::Float64) * lit(100))
            .alias("operating_margin"),
    ])
}
pub fn debt_to_equity(lf: LazyFrame) -> LazyFrame {
    lf.with_columns([
        when(col("equity").is_null().or(col("equity").lt_eq(lit(0))))
            .then(lit(NULL))
            .otherwise(col("debt") / col("equity").cast(DataType::Float64))
            .alias("debt_to_equity"),
    ])
}
