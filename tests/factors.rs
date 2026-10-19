mod common;

use common::assert_numbers;
use financial_factor_calculator::factors::*;
use polars::prelude::*;

#[test]
fn ratios_use_fractional_division_and_preserve_losses_and_nulls() -> PolarsResult<()> {
    let input = df!(
        "equity" => [Some(8i64), Some(8), Some(8), Some(0), Some(-8), None, Some(8)],
        "revenue" => [Some(8i64), Some(8), Some(8), Some(0), Some(-8), None, Some(8)],
        "netinc" => [Some(1i64), Some(-1), Some(0), Some(1), Some(1), Some(1), None],
        "opinc" => [Some(1i64), Some(-1), Some(0), Some(1), Some(1), Some(1), None],
        "debt" => [Some(1i64), Some(-1), Some(0), Some(1), Some(1), Some(1), None]
    )?;
    let result = debt_to_equity(operating_margin(roe(input.lazy()))).collect()?;
    let percentages = [Some(12.5), Some(-12.5), Some(0.0), None, None, None, None];
    assert_numbers(&result, "roe", &percentages);
    assert_numbers(&result, "operating_margin", &percentages);
    assert_numbers(
        &result,
        "debt_to_equity",
        &[Some(0.125), Some(-0.125), Some(0.0), None, None, None, None],
    );
    assert_eq!(result.column("equity")?.i64()?.get(0), Some(8));
    Ok(())
}

#[test]
fn calculated_ratios_replace_existing_vendor_values() -> PolarsResult<()> {
    let input = df!("equity" => [8i64], "netinc" => [1i64], "roe" => [999.0])?;
    let result = roe(input.lazy()).collect()?;
    assert_eq!(result.width(), 3);
    assert_numbers(&result, "roe", &[Some(12.5)]);
    Ok(())
}

#[test]
fn roic_applies_effective_tax_rate_before_dividing_by_capital() -> PolarsResult<()> {
    let input = df!(
        "ebt" => [100i64, 100, 100, 100],
        "taxexp" => [25i64, 0, -10, 120],
        "opinc" => [80i64, -80, 80, 80],
        "invcapavg" => [400i64, 400, 400, 400]
    )?;
    let result = roic(input.lazy()).collect()?;
    assert_numbers(
        &result,
        "effective_tax_rate",
        &[Some(0.25), Some(0.0), Some(-0.1), Some(1.2)],
    );
    assert_numbers(
        &result,
        "nopat",
        &[Some(60.0), Some(-80.0), Some(88.0), Some(-16.0)],
    );
    assert_numbers(
        &result,
        "roic",
        &[Some(15.0), Some(-20.0), Some(22.0), Some(-4.0)],
    );
    Ok(())
}

#[test]
fn roic_returns_null_for_invalid_tax_base_capital_or_missing_inputs() -> PolarsResult<()> {
    let input = df!(
        "ebt" => [Some(0i64), Some(-1), None, Some(100), Some(100), Some(100), Some(100), Some(100)],
        "taxexp" => [Some(25i64), Some(25), Some(25), Some(25), Some(25), Some(25), None, Some(25)],
        "opinc" => [Some(80i64), Some(80), Some(80), Some(80), Some(80), Some(80), Some(80), None],
        "invcapavg" => [Some(400i64), Some(400), Some(400), Some(0), Some(-400), None, Some(400), Some(400)]
    )?;
    assert_numbers(&roic(input.lazy()).collect()?, "roic", &[None; 8]);
    Ok(())
}

#[test]
fn yoy_sorts_unsorted_rows_and_keeps_company_histories_separate() -> PolarsResult<()> {
    let input = df!(
        "ticker" => ["B", "A", "B", "A", "A"],
        "calendardate" => ["2023-12-31", "2024-12-31", "2022-12-31", "2022-12-31", "2023-12-31"],
        "revenue" => [30i64, 90, 40, 100, 120]
    )?;
    let result = yoy_revenue_growth(input.lazy())?.collect()?;
    assert_eq!(common::tickers(&result), ["A", "A", "A", "B", "B"]);
    assert_eq!(result.column("calendardate")?.dtype(), &DataType::Date);
    assert_numbers(
        &result,
        "yoy_revenue_growth",
        &[None, Some(20.0), Some(-25.0), None, Some(-25.0)],
    );
    Ok(())
}

#[test]
fn yoy_rejects_year_gaps_and_nonpositive_or_missing_baselines() -> PolarsResult<()> {
    let input = df!(
        "ticker" => ["A", "A", "A", "A", "A", "A", "A", "A"],
        "calendardate" => ["2016-12-31", "2018-12-31", "2019-12-31", "2020-12-31", "2021-12-31", "2022-12-31", "2023-12-31", "2024-12-31"],
        "revenue" => [Some(100i64), Some(200), Some(0), Some(-10), None, Some(50), Some(50), Some(0)]
    )?;
    let result = yoy_revenue_growth(input.lazy())?.collect()?;
    assert_numbers(
        &result,
        "yoy_revenue_growth",
        &[
            None,
            None,
            Some(-100.0),
            None,
            None,
            None,
            Some(0.0),
            Some(-100.0),
        ],
    );
    Ok(())
}

#[test]
fn yoy_does_not_compare_two_periods_in_the_same_year() -> PolarsResult<()> {
    let input = df!("ticker" => ["A", "A"], "calendardate" => ["2023-03-31", "2023-12-31"], "revenue" => [100i64, 150])?;
    assert_numbers(
        &yoy_revenue_growth(input.lazy())?.collect()?,
        "yoy_revenue_growth",
        &[None, None],
    );
    Ok(())
}

#[test]
fn yoy_rejects_malformed_dates_on_collection() -> PolarsResult<()> {
    let input = df!("ticker" => ["A"], "calendardate" => ["not-a-date"], "revenue" => [100i64])?;
    assert!(yoy_revenue_growth(input.lazy())?.collect().is_err());
    Ok(())
}

#[test]
fn missing_required_financial_column_is_an_error() -> PolarsResult<()> {
    let input = df!("equity" => [100i64])?;
    assert!(roe(input.lazy()).collect().is_err());
    Ok(())
}
