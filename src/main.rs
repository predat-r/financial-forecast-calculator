use financial_factor_calculator::factors::{debt_to_equity, operating_margin, roe};
use financial_factor_calculator::reader::read_from_csv;
use polars::prelude::{PlRefPath, col};

fn main() {
    let lf = match read_from_csv(PlRefPath::from("data/sharadar_sf1.csv")) {
        Ok(lf) => lf,
        Err(e) => {
            eprintln!("Failed to read CSV: {}", e);
            return;
        }
    };
    let roe_lf = roe(lf);
    let operating_margin_lf = operating_margin(roe_lf);
    let debt_to_equity_lf = debt_to_equity(operating_margin_lf).select([
        col("ticker"),
        col("calendardate"),
        col("debt"),
        col("equity"),
        col("revenue"),
        col("netinc"),
        col("roe"),
        col("operating_margin"),
        col("debt_to_equity"),
    ]);
    let df = match debt_to_equity_lf.collect() {
        Ok(df) => df,
        Err(e) => {
            eprintln!("Unable to calculate factors : {e}");
            return;
        }
    };
    println!("{}", df);

    println!("Hello, world!");
}
