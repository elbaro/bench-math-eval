use std::collections::BTreeMap;

use approx::assert_relative_eq;
use criterion::{BenchmarkId, Criterion};
use evalexpr::ContextWithMutableVariables;

static NUMBERS: &[(&str, f64)] = &[
    ("BTC", 3.0),
    ("ETH", 3.5),
    ("XRP", 2.0),
    ("BTC_USDT", 515.0),
    ("USDT_USD", 1.010),
    ("ETH_USD", 20.23),
    ("BTC_USD", 516.2),
];

fn eval_shunting(e: &str) -> f64 {
    let expr = shunting::ShuntingParser::parse_str(e).unwrap();
    let mut context = shunting::MathContext::new();
    for (k, v) in NUMBERS {
        context.setvar(k, *v);
    }
    context.eval(&expr).unwrap()
}

fn eval_thin_shunting(e: &str) -> f64 {
    let expr = thin_shunting::ShuntingParser::parse_str(e).unwrap();
    let mut context = thin_shunting::MathContext::new();
    for (k, v) in NUMBERS {
        context.setvar(k, *v);
    }
    context.eval(&expr).unwrap()
}

fn eval_mexprp(e: &str) -> f64 {
    let mut context = mexprp::Context::<f64>::new();
    for (k, v) in NUMBERS {
        context.set_var(k, *v);
    }
    mexprp::eval_ctx::<f64>(e, &context)
        .unwrap()
        .unwrap_single()
}

fn eval_meval(e: &str) -> f64 {
    let mut context = meval::Context::new();
    for (k, v) in NUMBERS {
        context.var(k.to_string(), *v);
    }
    meval::eval_str_with_context(e, context).unwrap()
}

fn eval_evalexpr(e: &str) -> f64 {
    let mut map = evalexpr::HashMapContext::new();
    for (k, v) in NUMBERS {
        map.set_value(k.to_string(), evalexpr::Value::Float(*v))
            .unwrap();
    }
    evalexpr::eval_float_with_context(e, &map).unwrap()
}
fn eval_mathew(e: &str) -> f64 {
    let mut eval = mathew::Eval::default();
    for (k, v) in NUMBERS {
        eval = eval.insert(k, &v.to_string()).unwrap();
    }
    eval.eval(e).unwrap() as f64
}

fn eval_fasteval(e: &str) -> f64 {
    let mut map: BTreeMap<String, f64> = NUMBERS.iter().map(|(k, v)| (k.to_string(), *v)).collect();
    fasteval::ez_eval(e, &mut map).unwrap()
}

pub fn bench_variables(c: &mut Criterion) {
    let mut group = c.benchmark_group("variables");
    for (i, (e, answer)) in [
        ("(BTC + ETH + XRP) / 3", 2.8 + 1.0 / 30.0),
        ("BTC_USDT * USDT_USD / ETH_USD", 25.711814137419672),
        ("BTC_USD / USDT_USD", 511.0891089108911),
    ]
    .into_iter()
    .enumerate()
    {
        group.bench_with_input(BenchmarkId::new("shunting", i), e, |b, e| {
            b.iter(|| assert_relative_eq!(eval_shunting(e), answer, max_relative = 1e-8))
        });
        group.bench_with_input(BenchmarkId::new("thin_shunting", i), e, |b, e| {
            b.iter(|| assert_relative_eq!(eval_thin_shunting(e), answer, max_relative = 1e-8))
        });
        group.bench_with_input(BenchmarkId::new("mexprp", i), e, |b, e| {
            b.iter(|| assert_relative_eq!(eval_mexprp(e), answer, max_relative = 1e-8))
        });
        group.bench_with_input(BenchmarkId::new("meval", i), e, |b, e| {
            b.iter(|| assert_relative_eq!(eval_meval(e), answer, max_relative = 1e-8))
        });
        group.bench_with_input(BenchmarkId::new("evalexpr", i), e, |b, e| {
            b.iter(|| assert_relative_eq!(eval_evalexpr(e), answer, max_relative = 1e-8))
        });
        group.bench_with_input(BenchmarkId::new("mathew", i), e, |b, e| {
            b.iter(|| assert_relative_eq!(eval_mathew(e), answer, max_relative = 1e-8))
        });

        group.bench_with_input(BenchmarkId::new("fasteval", i), e, |b, e| {
            b.iter(|| assert_relative_eq!(eval_fasteval(e), answer, max_relative = 1e-8))
        });
    }
    group.finish();
}
