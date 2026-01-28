use anyhow::{anyhow, Context, Result};
use std::io;

fn main() {
    divan::main();
}

#[divan::bench]
fn create_simple_error() -> Result<()> {
    Err(anyhow!("simple error message"))
}

#[divan::bench]
fn create_error_with_context() -> Result<()> {
    let result: io::Result<()> = Err(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    result.context("Failed to read configuration")?;
    Ok(())
}

#[divan::bench]
fn create_error_with_source() -> Result<()> {
    let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
    Err(anyhow::Error::new(io_error).context("Operation failed"))
}

#[divan::bench]
fn downcast_error() {
    divan::black_box({
        let error = anyhow::Error::new(io::Error::new(io::ErrorKind::NotFound, "not found"));
        let _downcast = error.downcast_ref::<io::Error>();
    });
}

#[divan::bench(args = [1, 10, 100])]
fn error_chain(depth: usize) -> Result<()> {
    fn create_nested_error(depth: usize) -> Result<()> {
        if depth == 0 {
            Err(anyhow!("base error"))
        } else {
            create_nested_error(depth - 1).context(format!("layer {}", depth))
        }
    }
    
    create_nested_error(depth)
}
