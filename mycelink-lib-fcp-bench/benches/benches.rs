#![cfg(test)]

use criterion::{criterion_group, criterion_main, Criterion};

mod insert_benches;

criterion_group!(name = inserts; config = Criterion::default().sample_size(10); targets = insert_benches::usk_bench::usk_bench, insert_benches::ssk_bench::ssk_bench, insert_benches::ksk_bench::ksk_bench);
criterion_main!(inserts);
