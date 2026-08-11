use braided::{BraidResult, braid};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn compute_product(left: &BraidResult, right: &BraidResult) {
    let _ = left * right;
}

fn get_inputs() -> [((BraidResult, BraidResult), &'static str); 6] {
    [
        ((braid![(); [1; 1]], braid![(); [2; -1]]), "1, 1"),
        (
            (
                braid![(); [1; 1]],
                braid![(); [2; -1], [1 => 3; 3], [1; -1]],
            ),
            "1, 5",
        ),
        (
            (
                braid![(); [1; 1]],
                braid![(); [2; -1], [1 => 3; 2], [2; 2], [1 => 2; 20]],
            ),
            "1, 25",
        ),
        (
            (
                braid![(); [1; 1], [1 => 3; 3], [1; -1]],
                braid![(); [2; -1], [1 => 3; 2], [2; 2]],
            ),
            "5, 5",
        ),
        (
            (
                braid![(); [1; 1], [1 => 3; 3], [1; -1]],
                braid![(); [2; -1], [1 => 3; 2], [2; 2], [1 => 2; 20]],
            ),
            "5, 25",
        ),
        (
            (
                braid![(); [1; 1], [1 => 3; 3], [1; -1], [1 => 3; -20]],
                braid![(); [2; -1], [1 => 3; 2], [2; 2], [1 => 2; 20]],
            ),
            "25, 25",
        ),
    ]
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Product by Operand Letter Length");
    for (inputs, id) in get_inputs() {
        group.bench_with_input(BenchmarkId::new("Braid Product", id), &inputs, |b, i| {
            b.iter(|| compute_product(&i.0, &i.1))
        });
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
