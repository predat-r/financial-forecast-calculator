# Financial Factor Calculator

A small command-line application built in Rust using Polars to calculate and explore financial metrics from company
financial statements.

I built this project to get hands-on experience with data processing in Rust, particularly working with Polars and its
lazy query API.

## What it does

The application reads financial data from a CSV file, calculates financial factors, and lets you explore the results
through a simple interactive menu.

It currently supports five metrics:

- **Return on Equity (ROE)** — How much profit a company generates relative to its shareholders' equity.
- **Return on Invested Capital (ROIC)** — How efficiently a company generates returns from invested capital.
- **Operating Margin** — The percentage of revenue remaining after operating expenses.
- **Debt-to-Equity** — A measure of how much debt a company has relative to its equity.
- **Year-over-Year Revenue Growth** — How much a company's revenue has grown or declined compared with the previous
  year.

You can also rank companies by a particular metric, compare companies, view a company's historical results, filter data
by year, and export results to CSV.

## Tech stack

- **Rust** — Application logic and CLI
- **Polars** — Reading, transforming, and analyzing financial data
- **Sharadar SF1** — Sample financial statement data

## Running the project

Make sure Rust and Cargo are installed, then clone the repository and run:

```bash
cargo run
```

The application uses the sample CSV data located at:

```text
data/sharadar_sf1.csv
```

Follow the prompts in the terminal to select an analysis.

## Project structure

```text
src/
├── main.rs       # CLI and user interaction
├── lib.rs        # Module declarations
├── reader.rs     # Loading CSV data
├── factors.rs    # Financial factor calculations
├── analysis.rs   # Ranking, filtering, and comparisons
└── export.rs     # CSV export
```

## A few implementation details

The calculations use Polars' lazy API, allowing operations to be built as queries before execution.

Missing or invalid financial values are handled as nulls rather than forcing potentially misleading calculations. The
project also checks consecutive reporting years when calculating revenue growth.

## Why I built it

I wanted to learn Rust by building something practical rather than working through isolated exercises. Financial data
provided a useful way to explore DataFrames, expressions, filtering, sorting, and lazy evaluation with Polars.

This is a learning project, not an investment recommendation tool.