use financial_factor_calculator::factors::{
    debt_to_equity, operating_margin, roe, roic, yoy_revenue_growth,
};
use financial_factor_calculator::reader::read_from_csv;
use polars::prelude::{LazyFrame, PlRefPath, PolarsResult};
use std::io::{self, Write};

fn main() -> PolarsResult<()> {
    let lf = read_from_csv(PlRefPath::from("data/sharadar_sf1.csv"))?;

    let lf = calculate_factors(lf)?;

    loop {
        print_menu();

        match get_user_selection() {
            MenuActions::RankCompaniesByFactor => {
                let factor = ask("Enter financial factor: ");
                let year = ask("Enter reporting year: ");
                let limit = ask("Enter number of companies: ");
                let direction = ask("Descending? (yes/no): ");

                // TODO: Parse inputs and call top_companies_by_factor()
                println!("Ranking: {factor}, {year}, {limit}, {direction}");
            }

            MenuActions::CompareCompanies => {
                let tickers = ask("Enter tickers separated by commas: ");
                let year = ask("Enter reporting year: ");

                // TODO: Parse inputs and call compare_companies()
                println!("Comparing: {tickers} for {year}");
            }

            MenuActions::ViewCompanyFactorHistory => {
                let ticker = ask("Enter company ticker: ");
                let factor = ask("Enter financial factor: ");

                // TODO: Call factor_history()
                println!("History: {ticker}, {factor}");
            }

            MenuActions::ViewFinancialFiguresForAYear => {
                let year = ask("Enter reporting year: ");

                // TODO: Parse year and call figures_for_year()
                println!("Financial figures for {year}");
            }

            MenuActions::Exit => {
                println!("Goodbye.");
                break;
            }

            MenuActions::InvalidInput => {
                println!("Invalid selection. Please try again.");
            }
        }
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

fn ask(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn get_user_selection() -> MenuActions {
    match ask("Enter your choice: ").as_str() {
        "1" => MenuActions::RankCompaniesByFactor,
        "2" => MenuActions::CompareCompanies,
        "3" => MenuActions::ViewCompanyFactorHistory,
        "4" => MenuActions::ViewFinancialFiguresForAYear,
        "5" => MenuActions::Exit,
        _ => MenuActions::InvalidInput,
    }
}

fn print_menu() {
    println!("\n===== Financial Factor Calculator =====");
    println!("1. Rank companies by financial factor");
    println!("2. Compare companies");
    println!("3. View company factor history");
    println!("4. View financial figures for a year");
    println!("5. Exit");
}

enum MenuActions {
    RankCompaniesByFactor,
    CompareCompanies,
    ViewCompanyFactorHistory,
    ViewFinancialFiguresForAYear,
    InvalidInput,
    Exit,
}
