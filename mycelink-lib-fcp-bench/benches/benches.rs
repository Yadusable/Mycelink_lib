#![cfg(test)]

use criterion::{criterion_group, criterion_main, Criterion};

mod usk_bench;

criterion_group!(name = inserts; config = Criterion::default()/*.sample_size(10)*/; targets = usk_bench::usk_bench);
criterion_main!(inserts);
