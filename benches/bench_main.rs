use criterion::{criterion_group, criterion_main};

mod arithmetic;
mod compiled;
mod variables;

criterion_group!(arithmetic, arithmetic::bench_arithmetic);
criterion_group!(variables, variables::bench_variables);
criterion_group!(compiled, compiled::bench_compiled);
criterion_main! {arithmetic, variables, compiled}
