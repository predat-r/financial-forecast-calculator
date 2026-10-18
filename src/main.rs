use crate::factors::roe;
use financial_factor_calculator::reader::read_from_csv;
use polars::error::PolarsResult;
use polars::prelude::{PlRefPath, col};

mod analysis;
mod factors;

fn main() {
    let lf = match read_from_csv(PlRefPath::from("data/sharadar_sf1.csv")) {
        Ok(lf) => lf,
        Err(e) => {
            eprintln!("Failed to read CSV: {}", e);
            return;
        }
    };
    let df = roe(lf)
        .select([
            col("ticker"),
            col("calendardate"),
            col("netinc"),
            col("equity"),
            col("roe"),
        ])
        .collect()
        .expect("Failed to calculate ROE");

    println!("{}", df);

    println!("Hello, world!");
}
