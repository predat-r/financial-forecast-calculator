#![allow(dead_code)]

use polars::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

pub fn assert_numbers(df: &DataFrame, column: &str, expected: &[Option<f64>]) {
    let actual = df.column(column).unwrap().f64().unwrap();
    assert_eq!(actual.len(), expected.len(), "{column}: row count");
    for (row, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        match (actual, expected) {
            (Some(actual), Some(expected)) => assert!(
                actual.is_finite() && (actual - expected).abs() <= 1e-9 * expected.abs().max(1.0),
                "{column}, row {row}: expected {expected}, got {actual}"
            ),
            (None, None) => (),
            _ => panic!("{column}, row {row}: expected {expected:?}, got {actual:?}"),
        }
    }
}

pub fn tickers(df: &DataFrame) -> Vec<&str> {
    df.column("ticker")
        .unwrap()
        .str()
        .unwrap()
        .iter()
        .map(|ticker| ticker.expect("fixture ticker should not be null"))
        .collect()
}

// Each test owns a separate directory, including when tests run concurrently.
pub struct TestDir(PathBuf);

impl TestDir {
    pub fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        loop {
            let path = std::env::temp_dir().join(format!(
                "financial-factor-tests-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match std::fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("Cannot create test directory: {error}"),
            }
        }
    }

    pub fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

pub fn read_csv(path: &Path) -> PolarsResult<DataFrame> {
    financial_factor_calculator::reader::read_from_csv(PlRefPath::from(path.to_str().unwrap()))?
        .collect()
}
