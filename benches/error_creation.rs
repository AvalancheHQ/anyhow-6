use anyhow::{anyhow, Context, Error, Result};
use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use std::io;

fn error_from_string(c: &mut Criterion) {
    c.bench_function("error_from_string", |b| {
        b.iter(|| Error::msg("something went wrong"))
    });
}

fn error_from_anyhow_macro(c: &mut Criterion) {
    c.bench_function("error_from_anyhow_macro", |b| {
        b.iter(|| anyhow!("error with {} interpolation", "string"))
    });
}

fn error_from_io_error(c: &mut Criterion) {
    c.bench_function("error_from_io_error", |b| {
        b.iter(|| {
            let err = io::Error::new(io::ErrorKind::NotFound, "file not found");
            Error::new(err)
        })
    });
}

fn error_with_context(c: &mut Criterion) {
    c.bench_function("error_with_context", |b| {
        b.iter(|| {
            let result: Result<()> = Err(io::Error::new(io::ErrorKind::NotFound, "file not found").into());
            result.context("failed to read config file").unwrap_err()
        })
    });
}

fn error_with_lazy_context(c: &mut Criterion) {
    c.bench_function("error_with_lazy_context", |b| {
        b.iter(|| {
            let result: Result<()> = Err(io::Error::new(io::ErrorKind::NotFound, "file not found").into());
            result.with_context(|| format!("failed to read file at path")).unwrap_err()
        })
    });
}

fn error_chain_creation(c: &mut Criterion) {
    c.bench_function("error_chain_creation", |b| {
        b.iter(|| {
            let result: Result<()> = Err(io::Error::new(io::ErrorKind::NotFound, "file not found").into());
            result
                .context("failed to read config")
                .context("failed to initialize app")
                .context("startup failed")
                .unwrap_err()
        })
    });
}

criterion_group!(
    benches,
    error_from_string,
    error_from_anyhow_macro,
    error_from_io_error,
    error_with_context,
    error_with_lazy_context,
    error_chain_creation,
);
criterion_main!(benches);
