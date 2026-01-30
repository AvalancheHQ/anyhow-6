use anyhow::{anyhow, Context, Result};
use std::io;

fn main() {
    divan::main();
}

// Benchmark error creation using anyhow! macro
#[divan::bench]
fn create_anyhow_error() -> anyhow::Error {
    anyhow!("Something went wrong")
}

// Benchmark error creation with formatting
#[divan::bench]
fn create_formatted_error() -> anyhow::Error {
    let value = 42;
    anyhow!("Error occurred with value: {}", value)
}

// Benchmark Result with context
#[divan::bench]
fn add_context() -> Result<()> {
    let result: io::Result<()> = Err(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    result.context("Failed to read configuration")?;
    Ok(())
}

// Benchmark Result with lazy context
#[divan::bench]
fn add_lazy_context() -> Result<()> {
    let result: io::Result<()> = Err(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    let path = "/etc/config.json";
    result.with_context(|| format!("Failed to read config from {}", path))?;
    Ok(())
}

// Benchmark error conversion from std error
#[divan::bench]
fn convert_io_error() -> Result<()> {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    Err(io_err)?
}

// Benchmark error downcast
#[divan::bench]
fn downcast_error() -> bool {
    let error = anyhow!(io::Error::new(io::ErrorKind::NotFound, "not found"));
    error.downcast_ref::<io::Error>().is_some()
}

// Benchmark chain iteration
#[divan::bench]
fn iterate_error_chain() -> usize {
    let error = io::Error::new(io::ErrorKind::NotFound, "file.txt not found");
    let error = anyhow::Error::from(error).context("Failed to load file");
    error.chain().count()
}

// Benchmark error display formatting
#[divan::bench]
fn format_error_display() -> String {
    let error = anyhow!("Something went wrong");
    format!("{}", error)
}

// Benchmark error debug formatting
#[divan::bench]
fn format_error_debug() -> String {
    let error = anyhow!("Something went wrong");
    format!("{:?}", error)
}
