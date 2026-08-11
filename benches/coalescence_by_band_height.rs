use braided::{Braid, braid};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn coalesce(braid: &Braid) {
    let _ = braid.coalesce();
}

fn get_inputs() -> [(Braid, &'static str); 4] {
    [
        (
            braid![(); [4; -1], [4; -1], [4; -1], [4; -1], [4; -1], [5; 1], [4; 1], [4; 1], [4; 1], [4; 1], [4; 1]].clone_unwrap(),
            "height 1",
        ),
        (
            braid![(); [3; -1], [3; -1], [3; -1], [3; -1], [4; -1], [5; 1], [4; 1], [3; 1], [3; 1], [3; 1], [3; 1]].clone_unwrap(),
            "height 2",
        ),
        (
            braid![(); [2; -1], [2; -1], [2; -1], [3; -1], [4; -1], [5; 1], [4; 1], [3; 1], [2; 1], [2; 1], [2; 1]].clone_unwrap(),
            "height 3",
        ),
        (
            braid![(); [1; -1], [1; -1], [2; -1], [3; -1], [4; -1], [5; 1], [4; 1], [3; 1], [2; 1], [1; 1], [1; 1]].clone_unwrap(),
            "height 4",
        ),
    ]
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Coalescence by Band Height");
    for (br, id) in get_inputs() {
        group.bench_with_input(BenchmarkId::new("Coalescence", id), &br, |b, br| {
            b.iter(|| coalesce(br))
        });
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
