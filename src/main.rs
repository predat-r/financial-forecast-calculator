use financial_factor_calculator::analysis::{
    compare_companies, factor_history, figures_for_year, top_companies_by_factor,
};
use financial_factor_calculator::export::export_to_csv;
use financial_factor_calculator::factors::{
    debt_to_equity, operating_margin, roe, roic, yoy_revenue_growth,
};
use financial_factor_calculator::reader::read_from_csv;
use polars::prelude::{LazyFrame, PlRefPath, PolarsResult};
use std::io::{self, Write};

enum MenuActions {
    RankCompaniesByFactor,
    CompareCompanies,
    ViewCompanyFactorHistory,
    ViewFinancialFiguresForAYear,
    InvalidInput,
}

fn main() -> PolarsResult<()> {
    let lf = read_from_csv(PlRefPath::from("data/sharadar_sf1.csv"))?;
    let lf = calculate_factors(lf)?;

    println!("\n===== Financial Factor Calculator =====");
    println!("1. Rank companies by factor");
    println!("2. Compare companies");
    println!("3. View company factor history");
    println!("4. View all figures for a year");

    let result = match get_user_selection() {
        MenuActions::RankCompaniesByFactor => {
            let factor = ask("Factor: ");
            let year = ask("Year: ").parse::<i32>().expect("Invalid year");
            let limit = ask("Number of companies: ")
                .parse::<usize>()
                .expect("Invalid limit");
            let descending = ask("Descending? (y/n): ").eq_ignore_ascii_case("y");

            let lf = figures_for_year(lf, year);
            top_companies_by_factor(lf, &factor, limit, descending)?
        }

        MenuActions::CompareCompanies => {
            let input = ask("Tickers (comma-separated): ");
            let year = ask("Year: ").parse::<i32>().expect("Invalid year");

            let tickers: Vec<&str> = input.split(',').map(str::trim).collect();

            compare_companies(figures_for_year(lf, year), tickers)
        }

        MenuActions::ViewCompanyFactorHistory => {
            let ticker = ask("Ticker: ");
            let factor = ask("Factor: ");

            factor_history(lf, &ticker, &factor)?
        }

        MenuActions::ViewFinancialFiguresForAYear => {
            let year = ask("Year: ").parse::<i32>().expect("Invalid year");
            figures_for_year(lf, year)
        }

        MenuActions::InvalidInput => {
            println!("Invalid selection.");
            return Ok(());
        }
    };

    let df = result.collect()?;
    println!("{df}");

    if ask("Export result to CSV? (y/n): ").eq_ignore_ascii_case("y") {
        let filename = ask("Enter output filename (e.g. results.csv): ");

        export_to_csv(df, filename.into())?;

        println!("CSV exported successfully.");
    }

    Ok(())
}

fn calculate_factors(lf: LazyFrame) -> PolarsResult<LazyFrame> {
    let lf = roe(lf);
    let lf = operating_margin(lf);
    let lf = debt_to_equity(lf);
    let lf = yoy_revenue_growth(lf)?;
    Ok(roic(lf))
}

fn get_user_selection() -> MenuActions {
    match ask("Choose an analysis: ").as_str() {
        "1" => MenuActions::RankCompaniesByFactor,
        "2" => MenuActions::CompareCompanies,
        "3" => MenuActions::ViewCompanyFactorHistory,
        "4" => MenuActions::ViewFinancialFiguresForAYear,
        _ => MenuActions::InvalidInput,
    }
}

fn ask(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}
