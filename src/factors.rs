use polars::prelude::{
    DataType, LazyFrame, NULL, PolarsResult, SortMultipleOptions, StrptimeOptions, col, lit, when,
};
use std::ops::{Mul, Sub};

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
            col("calendardate")
                .str()
                .to_date(StrptimeOptions::default())
                .alias("calendardate"),
        )
        .with_columns([
            col("calendardate")
                .shift(lit(1))
                .over([col("ticker")])?
                .alias("previous_year_date"),
            col("revenue")
                .shift(lit(1))
                .over([col("ticker")])?
                .alias("previous_year_revenue"),
        ])
        .with_columns([when(
            col("previous_year_revenue")
                .is_null()
                .or(col("previous_year_revenue").lt_eq(lit(0)))
                .or(
                    (col("calendardate").dt().year() - col("previous_year_date").dt().year())
                        .neq(lit(1)),
                ),
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

pub fn roic(lf: LazyFrame) -> LazyFrame {
    lf.with_column(
        when(col("ebt").is_null().or(col("ebt").lt_eq(lit(0))))
            .then(lit(NULL))
            .otherwise(col("taxexp") / col("ebt").cast(DataType::Float64))
            .alias("effective_tax_rate"),
    )
    .with_columns([col("opinc")
        .mul(lit(1).sub(col("effective_tax_rate")))
        .alias("nopat")])
    .with_columns([when(
        col("invcapavg")
            .is_null()
            .or(col("invcapavg").lt_eq(lit(0))),
    )
    .then(lit(NULL))
    .otherwise(col("nopat") / col("invcapavg").cast(DataType::Float64) * lit(100))
    .alias("roic")])
}
