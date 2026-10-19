mod common;

use common::{TestDir, assert_numbers, read_csv, tickers};
use financial_factor_calculator::analysis::{figures_for_year, top_companies_by_factor};
use financial_factor_calculator::export::export_to_csv;
use financial_factor_calculator::factors::*;
use financial_factor_calculator::reader::read_from_csv;
use polars::prelude::*;

#[test]
fn csv_round_trip_preserves_quoted_text_numbers_and_nulls() -> PolarsResult<()> {
    let dir = TestDir::new();
    let path = dir.path("round-trip.csv");
    let input = df!(
        "ticker" => ["ACME, Inc.", "Say \"Hello\"", "Two\nLines"],
        "revenue" => [Some(123i64), None, Some(-45)],
        "roe" => [Some(12.5), Some(-2.75), None]
    )?;
    export_to_csv(input.clone(), path.clone())?;
    assert!(read_csv(&path)?.equals_missing(&input));
    assert!(
        std::fs::read_to_string(path)
            .unwrap()
            .starts_with("ticker,revenue,roe\n")
    );
    Ok(())
}

#[test]
fn export_replaces_existing_file_without_leaving_trailing_data() -> PolarsResult<()> {
    let dir = TestDir::new();
    let path = dir.path("existing.csv");
    std::fs::write(
        &path,
        "old contents that are much longer than the exported CSV\n".repeat(10),
    )
    .unwrap();
    export_to_csv(df!("ticker" => ["A"])?, path.clone())?;
    assert_eq!(std::fs::read_to_string(path).unwrap(), "ticker\nA\n");
    Ok(())
}

#[test]
fn export_empty_results_retains_csv_headers() -> PolarsResult<()> {
    let dir = TestDir::new();
    let path = dir.path("empty.csv");
    export_to_csv(
        df!("ticker" => Vec::<&str>::new(), "roe" => Vec::<f64>::new())?,
        path.clone(),
    )?;
    assert_eq!(std::fs::read_to_string(path).unwrap(), "ticker,roe\n");
    Ok(())
}

#[test]
fn missing_input_and_invalid_output_paths_return_errors() -> PolarsResult<()> {
    let dir = TestDir::new();
    assert!(read_csv(&dir.path("missing.csv")).is_err());
    assert!(
        export_to_csv(
            df!("ticker" => ["A"])?,
            dir.path("missing-parent/output.csv")
        )
        .is_err()
    );
    assert!(export_to_csv(df!("ticker" => ["A"])?, dir.path(".")).is_err());
    Ok(())
}

#[test]
fn csv_to_factors_to_year_ranking_to_export() -> PolarsResult<()> {
    let dir = TestDir::new();
    let input = dir.path("financials.csv");
    std::fs::write(
        &input,
        concat!(
            "ticker,calendardate,equity,netinc,revenue,opinc,debt,ebt,taxexp,invcapavg\n",
            "A,2024-12-31,80,20,120,30,40,20,5,100\n",
            "B,2023-12-31,100,10,200,40,20,20,5,200\n",
            "A,2023-12-31,80,8,100,20,40,20,5,100\n",
            "B,2024-12-31,100,10,180,20,20,20,5,200\n"
        ),
    )
    .unwrap();
    let lf = read_from_csv(PlRefPath::from(input.to_str().unwrap()))?;
    let lf = roic(yoy_revenue_growth(debt_to_equity(operating_margin(roe(
        lf,
    ))))?);
    let ranked = top_companies_by_factor(figures_for_year(lf, 2024), "roe", 2, true)?.collect()?;
    assert_eq!(tickers(&ranked), ["A", "B"]);
    assert_numbers(&ranked, "roe", &[Some(25.0), Some(10.0)]);
    assert_numbers(
        &ranked,
        "operating_margin",
        &[Some(25.0), Some(100.0 / 9.0)],
    );
    assert_numbers(&ranked, "debt_to_equity", &[Some(0.5), Some(0.2)]);
    assert_numbers(&ranked, "yoy_revenue_growth", &[Some(20.0), Some(-10.0)]);
    assert_numbers(&ranked, "roic", &[Some(22.5), Some(7.5)]);
    let output = dir.path("ranked.csv");
    export_to_csv(ranked, output.clone())?;
    let exported = read_csv(&output)?;
    assert_eq!(tickers(&exported), ["A", "B"]);
    assert_numbers(&exported, "roe", &[Some(25.0), Some(10.0)]);
    assert_numbers(&exported, "yoy_revenue_growth", &[Some(20.0), Some(-10.0)]);
    Ok(())
}
