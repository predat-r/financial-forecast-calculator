use financial_factor_calculator::factors::{
    debt_to_equity, operating_margin, roe, yoy_revenue_growth,
};
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
    let roe_lf = roe(lf.clone());
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
    let _df = match debt_to_equity_lf.collect() {
        Ok(df) => df,
        Err(e) => {
            eprintln!("Unable to calculate factors : {e}");
            return;
        }
    };

    //only for testing yoy_revenue_growth separately
    let lf2 = match yoy_revenue_growth(lf) {
        Ok(lf) => lf.select([
            col("ticker"),
            col("calendardate"),
            col("revenue"),
            col("previous_year_revenue"),
            col("yoy_revenue_growth"),
        ]),
        Err(e) => {
            eprintln!("Error : {e}");
            return;
        }
    };
    let df2 = match lf2.collect() {
        Ok(df) => df,
        Err(e) => {
            eprintln!("Unable to calculate factors : {e}");
            return;
        }
    };
    println!("{}", df2);
}
