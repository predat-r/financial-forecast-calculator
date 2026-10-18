use financial_factor_calculator::analysis::{figures_for_year, top_companies_by_factor};
use financial_factor_calculator::factors::{
    debt_to_equity, operating_margin, roe, roic, yoy_revenue_growth,
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
    let roe_lf = roe(lf);
    let operating_margin_lf = operating_margin(roe_lf);
    let debt_to_equity_lf = debt_to_equity(operating_margin_lf);
    let yoy_revenue_growth_lf = match yoy_revenue_growth(debt_to_equity_lf) {
        Ok(lf) => lf,
        Err(e) => {
            eprintln!("Error : {e}");
            return;
        }
    };
    let lf = roic(yoy_revenue_growth_lf).select([
        col("ticker"),
        col("calendardate"),
        col("debt"),
        col("equity"),
        col("revenue"),
        col("netinc"),
        col("roe"),
        col("operating_margin"),
        col("debt_to_equity"),
        col("yoy_revenue_growth"),
        col("invcapavg"),
        col("roic"),
        col("nopat"),
    ]);
    let filtered_lf = figures_for_year(lf, 2023);
    let ranked_lf = top_companies_by_factor(filtered_lf.clone(), "roic", 5, true).expect(
        "Unsupported
    financial factor",
    );
    let ranked2_lf = top_companies_by_factor(filtered_lf, "yoy_revenue_growth", 5, true).expect(
        "Unsupported \
    financial factor",
    );
    let df = match ranked_lf.collect() {
        Ok(df) => df,
        Err(e) => {
            eprintln!("Unable to calculate factors : {e}");
            return;
        }
    };
    let df2 = match ranked2_lf.collect() {
        Ok(df) => df,
        Err(e) => {
            eprintln!("Unable to calculate factors : {e}");
            return;
        }
    };

    println!("{}", df);
    println!("{}", df2);
}
