use polars::prelude::{
    DataType, LazyFrame, NULL, PolarsResult, SortMultipleOptions, StrptimeOptions, col, lit, when,
};
use std::ops::{Mul, Sub};

/// Adds return on equity (`roe`) as net income divided by equity, in percent.
///
/// Rows with null or non-positive equity receive a null factor value.
pub fn roe(lf: LazyFrame) -> LazyFrame {
    lf.with_columns([
        when(col("equity").is_null().or(col("equity").lt_eq(lit(0))))
            .then(lit(NULL))
            .otherwise(col("netinc") / col("equity").cast(DataType::Float64) * lit(100))
            .alias("roe"),
    ])
}

/// Adds operating margin (`operating_margin`) as operating income divided by
/// revenue, in percent.
///
/// Rows with null or non-positive revenue receive a null factor value.
pub fn operating_margin(lf: LazyFrame) -> LazyFrame {
    lf.with_columns([
        when(col("revenue").is_null().or(col("revenue").lt_eq(lit(0))))
            .then(lit(NULL))
            .otherwise(col("opinc") / col("revenue").cast(DataType::Float64) * lit(100))
            .alias("operating_margin"),
    ])
}
/// Adds debt-to-equity (`debt_to_equity`) as debt divided by equity.
///
/// Rows with null or non-positive equity receive a null factor value.
pub fn debt_to_equity(lf: LazyFrame) -> LazyFrame {
    lf.with_columns([
        when(col("equity").is_null().or(col("equity").lt_eq(lit(0))))
            .then(lit(NULL))
            .otherwise(col("debt") / col("equity").cast(DataType::Float64))
            .alias("debt_to_equity"),
    ])
}

/// Adds year-over-year revenue growth (`yoy_revenue_growth`), in percent.
///
/// Records are ordered by ticker and calendar date. Growth is only calculated
/// when the preceding record for that ticker is exactly one calendar year
/// earlier and its revenue is positive; otherwise the value is null.
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

/// Adds return on invested capital (`roic`), in percent, along with the
/// intermediate `effective_tax_rate` and `nopat` columns.
///
/// The effective tax rate is tax expense divided by positive earnings before
/// tax. ROIC is null when average invested capital is null or non-positive.
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
