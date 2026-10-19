mod common;

use common::tickers;
use financial_factor_calculator::analysis::*;
use financial_factor_calculator::factors::yoy_revenue_growth;
use polars::prelude::*;

const FACTORS: [&str; 5] = [
    "roe",
    "roic",
    "yoy_revenue_growth",
    "debt_to_equity",
    "operating_margin",
];

fn fixture() -> PolarsResult<LazyFrame> {
    let df = df!(
        "ticker" => ["A", "B", "A", "C"],
        "calendardate" => ["2024-12-31", "2023-12-31", "2022-12-31", "2024-12-31"],
        "revenue" => [120i64, 100, 100, 80],
        "roe" => [Some(10.0), Some(30.0), Some(-5.0), None]
    )?;
    yoy_revenue_growth(df.lazy())
}

#[test]
fn ranking_supports_every_factor_and_both_directions() -> PolarsResult<()> {
    for factor in FACTORS {
        let lf = df!("ticker" => ["LOW", "NULL", "HIGH", "MID"], factor => [Some(-2.5), None, Some(30.0), Some(10.0)])?.lazy();
        let descending = top_companies_by_factor(lf.clone(), factor, 2, true)?.collect()?;
        assert_eq!(tickers(&descending), ["HIGH", "MID"], "{factor}");
        let ascending = top_companies_by_factor(lf, factor, 2, false)?.collect()?;
        assert_eq!(tickers(&ascending), ["LOW", "MID"], "{factor}");
    }
    Ok(())
}

#[test]
fn ranking_handles_zero_oversized_limits_and_all_null_factors() -> PolarsResult<()> {
    let lf = fixture()?;
    assert_eq!(
        top_companies_by_factor(lf.clone(), "roe", 0, true)?
            .collect()?
            .height(),
        0
    );
    assert_eq!(
        top_companies_by_factor(lf, "roe", 100, true)?
            .collect()?
            .height(),
        3
    );
    let all_null = df!("ticker" => ["A", "B"], "roe" => [None::<f64>, None])?.lazy();
    let result = top_companies_by_factor(all_null, "roe", 10, true)?.collect()?;
    assert_eq!(result.height(), 0);
    assert_eq!(result.width(), 2);
    Ok(())
}

#[test]
fn unsupported_factor_names_return_explicit_errors() -> PolarsResult<()> {
    for factor in ["", "revenue", "ROE", "roe ", "unknown"] {
        assert!(matches!(
            top_companies_by_factor(fixture()?, factor, 2, true),
            Err(PolarsError::InvalidOperation(_))
        ));
        assert!(matches!(
            factor_history(fixture()?, "A", factor),
            Err(PolarsError::InvalidOperation(_))
        ));
    }
    Ok(())
}

#[test]
fn supported_but_absent_factor_returns_collection_error() -> PolarsResult<()> {
    assert!(
        top_companies_by_factor(fixture()?, "roic", 2, true)?
            .collect()
            .is_err()
    );
    assert!(factor_history(fixture()?, "A", "roic")?.collect().is_err());
    Ok(())
}

#[test]
fn year_filter_selects_all_companies_and_preserves_schema() -> PolarsResult<()> {
    let lf = fixture()?;
    let result = figures_for_year(lf.clone(), 2024).collect()?;
    assert_eq!(tickers(&result), ["A", "C"]);
    let empty = figures_for_year(lf, 1999).collect()?;
    assert_eq!(empty.height(), 0);
    assert_eq!(empty.schema(), result.schema());
    Ok(())
}

#[test]
fn company_comparison_selects_all_periods_without_duplicating_requested_tickers() -> PolarsResult<()>
{
    let result = compare_companies(fixture()?, vec!["C", "A", "A", "UNKNOWN"]).collect()?;
    assert_eq!(tickers(&result), ["A", "A", "C"]);
    assert!(result.column("revenue").is_ok());
    Ok(())
}

#[test]
fn company_comparison_handles_empty_unknown_and_case_sensitive_requests() -> PolarsResult<()> {
    for requested in [vec![], vec!["UNKNOWN"], vec!["a"]] {
        assert_eq!(
            compare_companies(fixture()?, requested).collect()?.height(),
            0
        );
    }
    Ok(())
}

#[test]
fn history_supports_every_factor_and_sorts_dates_chronologically() -> PolarsResult<()> {
    for factor in FACTORS {
        let lf = df!(
            "ticker" => ["A", "B", "A", "A"],
            "calendardate" => ["2024-12-31", "2023-12-31", "2022-12-31", "2023-12-31"],
            factor => [Some(40.0), Some(999.0), Some(20.0), None],
            "extra" => [1i64, 2, 3, 4]
        )?
        .lazy();
        let result = factor_history(lf.clone(), "A", factor)?.collect()?;
        assert_eq!(tickers(&result), ["A", "A", "A"]);
        assert_eq!(
            result.get_column_names(),
            ["ticker", "calendardate", factor]
        );
        common::assert_numbers(&result, factor, &[Some(20.0), None, Some(40.0)]);
        let empty = factor_history(lf, "UNKNOWN", factor)?.collect()?;
        assert_eq!(empty.height(), 0);
        assert_eq!(empty.schema(), result.schema());
    }
    Ok(())
}
