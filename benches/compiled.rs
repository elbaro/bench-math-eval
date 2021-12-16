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

pub fn bench_compiled(c: &mut Criterion) {
    let mut group = c.benchmark_group("compiled");
    for (i, (e, answer)) in [
        ("(BTC + ETH + XRP) / 3", 2.8 + 1.0 / 30.0),
        ("BTC_USDT * USDT_USD / ETH_USD", 25.711814137419672),
        ("BTC_USD / USDT_USD", 511.0891089108911),
    ]
    .into_iter()
    .enumerate()
    {
        {
            let expr = shunting::ShuntingParser::parse_str(e).unwrap();
            let mut context = shunting::MathContext::new();
            for (k, v) in NUMBERS {
                context.setvar(k, *v);
            }
            group.bench_function(BenchmarkId::new("shunting", i), |b| {
                b.iter(|| {
                    assert_relative_eq!(context.eval(&expr).unwrap(), answer, max_relative = 1e-8)
                })
            });
        }
        {
            let expr = thin_shunting::ShuntingParser::parse_str(e).unwrap();
            let mut context = thin_shunting::MathContext::new();
            for (k, v) in NUMBERS {
                context.setvar(k, *v);
            }
            group.bench_function(BenchmarkId::new("thin_shunting", i), |b| {
                b.iter(|| {
                    assert_relative_eq!(context.eval(&expr).unwrap(), answer, max_relative = 1e-8)
                })
            });
        }
        {
            let mut context = mexprp::Context::<f64>::new();
            for (k, v) in NUMBERS {
                context.set_var(k, *v);
            }
            let term = mexprp::Term::<f64>::parse(e).unwrap();
            group.bench_function(BenchmarkId::new("mexprp", i), |b| {
                b.iter(|| {
                    assert_relative_eq!(
                        term.eval_ctx(&context).unwrap().unwrap_single(),
                        answer,
                        max_relative = 1e-8
                    )
                })
            });
        }
        {
            let expr: meval::Expr = e.parse().unwrap();
            let mut context = meval::Context::new();
            for (k, v) in NUMBERS {
                context.var(k.to_string(), *v);
            }
            group.bench_function(BenchmarkId::new("meval", i), |b| {
                b.iter(|| {
                    assert_relative_eq!(
                        expr.eval_with_context(&context).unwrap(),
                        answer,
                        max_relative = 1e-8
                    )
                })
            });
        }
        {
            let mut map = evalexpr::HashMapContext::new();
            for (k, v) in NUMBERS {
                map.set_value(k.to_string(), evalexpr::Value::Float(*v))
                    .unwrap();
            }
            let expr = evalexpr::build_operator_tree(e).unwrap();
            group.bench_function(BenchmarkId::new("evalexpr", i), |b| {
                b.iter(|| {
                    assert_relative_eq!(
                        expr.eval_float_with_context(&map).unwrap(),
                        answer,
                        max_relative = 1e-8
                    )
                })
            });
        }
        {
            use fasteval::{Compiler, Evaler};
            let mut slab = fasteval::Slab::new();
            let compiled = fasteval::Parser::new()
                .parse(e, &mut slab.ps)
                .unwrap()
                .from(&slab.ps)
                .compile(&slab.ps, &mut slab.cs);
            let mut map: BTreeMap<String, f64> =
                NUMBERS.iter().map(|(k, v)| (k.to_string(), *v)).collect();

            group.bench_function(BenchmarkId::new("fasteval", i), |b| {
                b.iter(|| {
                    assert_relative_eq!(
                        compiled.eval(&slab, &mut map).unwrap(),
                        answer,
                        max_relative = 1e-8
                    )
                })
            });
        }
    }
    group.finish();
}
