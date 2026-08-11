use braided::{Braid, braid};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn coalesce(braid: &Braid) {
    let _ = braid.coalesce();
}

fn get_inputs() -> [(Braid, &'static str); 4] {
    [
        (
            braid![(); [5; 1], [5; 1], [5; 1], [5; 1], [5; 1], [5; 1], [5; 1], [5; 1], [5; 1], [5; 1], [5; 1]].clone_unwrap(),
            "no bands",
        ),
        (
            braid![(); [5; -1], [5; -1], [5; -1], [5; -1], [4; -1], [5; 1], [4; 1], [5; 1], [5; 1], [5; 1], [5; 1]].clone_unwrap(),
            "one band",
        ),
        (
            braid![(); [4; -1], [4; -1], [5; 1], [4; 1], [4; 1], [5; 1], [4; -1], [4; -1], [5; 1], [4; 1], [4; 1]].clone_unwrap(),
            "two bands",
        ),
        (
            braid![(); [4; -1], [5; 1], [4; 1], [5; -1], [4; -1], [5; 1], [4; 1], [5; 1], [4; -1], [5; 1], [4; 1]].clone_unwrap(),
            "three bands",
        ),
    ]
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Coalescence by Number of Bands");
    for (br, id) in get_inputs() {
        group.bench_with_input(BenchmarkId::new("Coalescence", id), &br, |b, br| {
            b.iter(|| coalesce(br))
        });
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
