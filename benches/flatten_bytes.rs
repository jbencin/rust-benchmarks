use criterion::{black_box, criterion_group, criterion_main, Criterion};

const TEST_ITERATIONS: usize = 100;

#[inline(always)]
fn test_data() -> &'static [u8] {
    &[0x19, 0x76, 0xa9, 0x14]
}

fn bench_append(c: &mut Criterion) {
    c.bench_function("append", |b| {
        b.iter(|| {
            let mut output = Vec::new();
            for _ in 0..TEST_ITERATIONS {
                output.append(&mut Vec::from(test_data()));
            }
            black_box(output);
        })
    });
}

fn bench_extend_from_slice(c: &mut Criterion) {
    c.bench_function("extend_from_slice", |b| {
        b.iter(|| {
            let mut output = Vec::new();
            for _ in 0..TEST_ITERATIONS {
                output.extend_from_slice(test_data());
            }
            black_box(output);
        })
    });
}

fn bench_flat_map(c: &mut Criterion) {
    c.bench_function("flat_map", |b| {
        b.iter(|| {
            let output = (0..TEST_ITERATIONS)
                .flat_map(|_| test_data()) 
                .collect::<Vec<_>>();
            black_box(output);
        })
    });
}

fn bench_flat_map_vec(c: &mut Criterion) {
    c.bench_function("flat_map", |b| {
        b.iter(|| {
            let output = (0..TEST_ITERATIONS)
                .flat_map(|_| Vec::from(test_data())) 
                .collect::<Vec<_>>();
            black_box(output);
        })
    });
}

criterion_group!(benches, bench_append, bench_extend_from_slice, bench_flat_map, bench_flat_map_vec);
criterion_main!(benches);
