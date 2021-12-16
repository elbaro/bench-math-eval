use approx::assert_relative_eq;
use criterion::{BenchmarkId, Criterion};

fn eval_shunting(e: &str) -> f64 {
    let expr = shunting::ShuntingParser::parse_str(e).unwrap();
    shunting::MathContext::new().eval(&expr).unwrap()
}

fn eval_thin_shunting(e: &str) -> f64 {
    let expr = thin_shunting::ShuntingParser::parse_str(e).unwrap();
    thin_shunting::MathContext::new().eval(&expr).unwrap()
}

fn eval_mexprp(e: &str) -> f64 {
    mexprp::eval::<f64>(e).unwrap().unwrap_single()
}

fn eval_meval(e: &str) -> f64 {
    meval::eval_str(e).unwrap()
}

fn eval_kalk(e: &str) -> f64 {
    let mut context = kalk::parser::Context::new();
    let precision = 53;
    kalk::parser::eval(&mut context, e, precision)
        .unwrap()
        .unwrap()
        .to_f64()
}

fn eval_exmex(e: &str) -> f64 {
    exmex::eval_str::<f64>(e).unwrap()
}
fn eval_evalexpr(e: &str) -> f64 {
    evalexpr::eval(e).unwrap().as_number().unwrap()
}
fn eval_rsc(e: &str) -> f64 {
    let mut c = rsc::computer::Computer::<f64>::default();
    c.eval(e).unwrap()
}

fn eval_mathew(e: &str) -> f64 {
    mathew::Eval::default().eval(e).unwrap() as f64
}

fn eval_fasteval(e: &str) -> f64 {
    fasteval::ez_eval(e, &mut fasteval::EmptyNamespace).unwrap()
}

pub fn bench_arithmetic(c: &mut Criterion) {
    let mut group = c.benchmark_group("arithmetic");
    for (i, (e, answer)) in [
        ("1 + 2", 3.0),
        (
            "0 - (-4294.1235 + 353.5100 / (1521.551 - 1/12.751))",
            4293.891152729428,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        group.bench_with_input(BenchmarkId::new("shunting", i), e, |b, e| {
            b.iter(|| assert_relative_eq!(eval_shunting(e), answer, max_relative = 1e-8))
        });
        group.bench_with_input(BenchmarkId::new("thin_shunting", i), e, |b, e| {
            b.iter(|| assert_relative_eq!(eval_thin_shunting(e), answer, max_relative = 1e-8));
        });
        group.bench_with_input(BenchmarkId::new("mexprp", i), e, |b, e| {
            b.iter(|| assert_relative_eq!(eval_mexprp(e), answer, max_relative = 1e-8));
        });
        group.bench_with_input(BenchmarkId::new("meval", i), e, |b, e| {
            b.iter(|| assert_relative_eq!(eval_meval(e), answer, max_relative = 1e-8));
        });
        group.bench_with_input(BenchmarkId::new("kalk", i), e, |b, e| {
            b.iter(|| assert_relative_eq!(eval_kalk(e), answer, max_relative = 1e-8));
        });
        group.bench_with_input(BenchmarkId::new("exmex", i), e, |b, e| {
            b.iter(|| assert_relative_eq!(eval_exmex(e), answer, max_relative = 1e-8));
        });
        group.bench_with_input(BenchmarkId::new("evalexpr", i), e, |b, e| {
            b.iter(|| assert_relative_eq!(eval_evalexpr(e), answer, max_relative = 1e-8));
        });
        group.bench_with_input(BenchmarkId::new("rsc", i), e, |b, e| {
            b.iter(|| assert_relative_eq!(eval_rsc(e), answer, max_relative = 1e-8));
        });
        group.bench_with_input(BenchmarkId::new("mathew", i), e, |b, e| {
            b.iter(|| assert_relative_eq!(eval_mathew(e), answer, max_relative = 1e-8));
        });

        group.bench_with_input(BenchmarkId::new("fasteval", i), e, |b, e| {
            b.iter(|| assert_relative_eq!(eval_fasteval(e), answer, max_relative = 1e-8));
        });
    }
    group.finish();
}
