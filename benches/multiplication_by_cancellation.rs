use braided::{BraidResult, braid};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn compute_product(left: &BraidResult, right: &BraidResult) {
    let _ = left * right;
}

fn get_inputs() -> [((BraidResult, BraidResult), &'static str); 4] {
    [
        (
            (
                braid![(); [1; 1], [1 => 3; 3], [1; -1]],
                braid![(); [2; -1], [1 => 3; 2], [2; 2]],
            ),
            "no cancellation",
        ),
        (
            (
                braid![(); [1; 1], [1 => 3; 3], [2; 1]],
                braid![(); [2; -1], [1 => 3; 2], [2; 2]],
            ),
            "one cancelling pair",
        ),
        (
            (
                braid![(); [1; 1], [1 => 3; 3], [2; 1]],
                braid![(); [2; -1], [1 => 3; -2], [2; 2]],
            ),
            "three cancelling pairs",
        ),
        (
            (
                braid![(); [1; 1], [1 => 3; 3], [2; 1]],
                braid![(); [2; -1], [1 => 3; -3], [1; -1]],
            ),
            "full cancellation",
        ),
    ]
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Product by Presence of Cancellation");
    for (inputs, id) in get_inputs() {
        group.bench_with_input(BenchmarkId::new("Braid Product", id), &inputs, |b, i| {
            b.iter(|| compute_product(&i.0, &i.1))
        });
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
