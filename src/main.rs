use financial_factor_calculator::reader::read_from_csv;
use polars::prelude::PlRefPath;

mod analysis;
mod factors;

fn main() {
    match read_from_csv(PlRefPath::from("data/sharadar_sf1.csv")) {
        Ok(lf) => match lf.collect() {
            Ok(df) => {
                println!("{}", df)
            }
            Err(e) => {
                println!("{}", e)
            }
        },
        Err(e) => {
            println!("{}", e)
        }
    }
    println!("Hello, world!");
}
