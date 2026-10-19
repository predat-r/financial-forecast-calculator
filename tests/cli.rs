mod common;

use common::{TestDir, assert_numbers, read_csv, tickers};
use polars::prelude::*;
use std::io::Write;
use std::process::{Command, Output, Stdio};

fn fixture() -> TestDir {
    let dir = TestDir::new();
    std::fs::create_dir(dir.path("data")).unwrap();
    std::fs::write(
        dir.path("data/sharadar_sf1.csv"),
        concat!(
            "ticker,calendardate,equity,netinc,revenue,opinc,debt,ebt,taxexp,invcapavg\n",
            "A,2024-12-31,80,20,120,30,40,20,5,100\n",
            "B,2024-12-31,100,10,180,20,20,20,5,200\n",
            "A,2023-12-31,80,8,100,20,40,20,5,100\n"
        ),
    )
    .unwrap();
    dir
}

fn run(dir: &TestDir, input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_financial-factor-calculator"))
        .current_dir(dir.path("."))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn every_menu_action_exports_the_expected_result() -> PolarsResult<()> {
    let cases = [
        ("1\nroe\n2024\n1\ny\ny\noutput.csv\n", vec!["A"]),
        ("2\n B, A \n2024\ny\noutput.csv\n", vec!["A", "B"]),
        ("3\nA\nroe\ny\noutput.csv\n", vec!["A", "A"]),
        ("4\n2024\ny\noutput.csv\n", vec!["A", "B"]),
    ];
    for (index, (input, expected_tickers)) in cases.into_iter().enumerate() {
        let dir = fixture();
        let output = run(&dir, input);
        assert!(
            output.status.success(),
            "menu {}: {}",
            index + 1,
            String::from_utf8_lossy(&output.stderr)
        );
        let result = read_csv(&dir.path("output.csv"))?;
        assert_eq!(tickers(&result), expected_tickers, "menu {}", index + 1);
        if index == 2 {
            assert_eq!(result.get_column_names(), ["ticker", "calendardate", "roe"]);
            assert_numbers(&result, "roe", &[Some(10.0), Some(25.0)]);
        } else {
            assert_numbers(
                &result,
                "roe",
                if index == 0 {
                    &[Some(25.0)]
                } else {
                    &[Some(25.0), Some(10.0)]
                },
            );
        }
        assert!(String::from_utf8_lossy(&output.stdout).contains("CSV exported successfully."));
    }
    Ok(())
}

#[test]
fn declining_export_does_not_create_a_result_file() {
    let dir = fixture();
    let output = run(&dir, "4\n2024\nn\n");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(std::fs::read_dir(dir.path(".")).unwrap().count(), 1);
    assert!(!String::from_utf8_lossy(&output.stdout).contains("CSV exported successfully."));
}

#[test]
fn invalid_menu_selection_exits_cleanly() {
    let dir = fixture();
    let output = run(&dir, "99\n");
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Invalid selection."));
    assert_eq!(std::fs::read_dir(dir.path(".")).unwrap().count(), 1);
}
