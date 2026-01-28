use anyhow::{anyhow, Context, Error};
use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};
use std::io;

fn error_downcast_success(c: &mut Criterion) {
    c.bench_function("error_downcast_success", |b| {
        b.iter(|| {
            let error = Error::new(io::Error::new(io::ErrorKind::NotFound, "file not found"));
            black_box(error.downcast::<io::Error>().unwrap())
        })
    });
}

fn error_downcast_failure(c: &mut Criterion) {
    c.bench_function("error_downcast_failure", |b| {
        b.iter(|| {
            let error = anyhow!("string error");
            black_box(error.downcast::<io::Error>().unwrap_err())
        })
    });
}

fn error_downcast_ref(c: &mut Criterion) {
    let error = Error::new(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    c.bench_function("error_downcast_ref", |b| {
        b.iter(|| black_box(error.downcast_ref::<io::Error>().unwrap()))
    });
}

fn error_is_check(c: &mut Criterion) {
    let error = Error::new(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    c.bench_function("error_is_check", |b| {
        b.iter(|| black_box(error.is::<io::Error>()))
    });
}

fn error_display(c: &mut Criterion) {
    let error = anyhow!("error with {} interpolation", "string")
        .context("outer context");
    c.bench_function("error_display", |b| {
        b.iter(|| black_box(format!("{}", error)))
    });
}

fn error_debug(c: &mut Criterion) {
    let error = anyhow!("error with {} interpolation", "string")
        .context("outer context");
    c.bench_function("error_debug", |b| {
        b.iter(|| black_box(format!("{:?}", error)))
    });
}

fn error_chain_iteration(c: &mut Criterion) {
    let error: Error = io::Error::new(io::ErrorKind::NotFound, "file not found")
        .into();
    let error = error
        .context("failed to read config")
        .context("failed to initialize");
    
    c.bench_function("error_chain_iteration", |b| {
        b.iter(|| {
            let mut count = 0;
            for _cause in error.chain() {
                count += 1;
            }
            black_box(count)
        })
    });
}

fn error_root_cause(c: &mut Criterion) {
    let error: Error = io::Error::new(io::ErrorKind::NotFound, "file not found")
        .into();
    let error = error
        .context("failed to read config")
        .context("failed to initialize");
    
    c.bench_function("error_root_cause", |b| {
        b.iter(|| black_box(error.root_cause()))
    });
}

criterion_group!(
    benches,
    error_downcast_success,
    error_downcast_failure,
    error_downcast_ref,
    error_is_check,
    error_display,
    error_debug,
    error_chain_iteration,
    error_root_cause,
);
criterion_main!(benches);
