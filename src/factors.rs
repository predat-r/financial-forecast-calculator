use polars::prelude::{
    DataType, LazyFrame, NULL, PolarsResult, SortMultipleOptions, col, lit, when,
};
use std::ops::Mul;

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

pub fn yoy_revenue_growth(lf: LazyFrame) -> PolarsResult<LazyFrame> {
    let result = lf
        .sort(
            vec!["ticker", "calendardate"],
            SortMultipleOptions::default().with_order_descending(false),
        )
        .with_column(
            col("revenue")
                .shift(lit(1))
                .over([col("ticker")])?
                .alias("previous_year_revenue"),
        )
        .with_columns([when(
            col("previous_year_revenue")
                .is_null()
                .or(col("previous_year_revenue").lt_eq(lit(0))),
        )
        .then(lit(NULL))
        .otherwise(
            ((col("revenue") - col("previous_year_revenue"))
                / col("previous_year_revenue").cast(DataType::Float64))
            .mul(lit(100)),
        )
        .alias("yoy_revenue_growth")]);
    Ok(result)
}
