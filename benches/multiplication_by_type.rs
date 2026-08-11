use braided::{BraidResult, LetterResult, WordResult, braid, letter, word};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn letter_product(left: LetterResult, right: LetterResult) {
    let _ = left * right;
}

fn word_product(left: &WordResult, right: &WordResult) {
    let _ = left * right;
}

fn braid_product(left: &BraidResult, right: &BraidResult) {
    let _ = left * right;
}

fn get_letter_inputs() -> [((LetterResult, LetterResult), &'static str); 3] {
    [
        ((letter![1; +], letter![1; +]), "artin x artin"),
        ((letter![1; +], letter![1 => 3; -]), "artin x band"),
        ((letter![1 => 3; -], letter![1 => 3; -]), "band x band"),
    ]
}
fn get_word_inputs() -> [((WordResult, WordResult), &'static str); 3] {
    [
        ((word![[1; 1]], word![[1; 1]]), "artin x artin"),
        ((word![[1; 1]], word![[1 => 3; -1]]), "artin x band"),
        ((word![[1 => 3; -1]], word![[1 => 3; -1]]), "band x band"),
    ]
}
fn get_braid_inputs() -> [((BraidResult, BraidResult), &'static str); 3] {
    [
        ((braid![(); [1; 1]], braid![(); [1; 1]]), "artin x artin"),
        (
            (braid![(); [1; 1]], braid![(); [1 => 3; -1]]),
            "artin x band",
        ),
        (
            (braid![(); [1 => 3; -1]], braid![(); [1 => 3; -1]]),
            "band x band",
        ),
    ]
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Products by Operand Type");
    for (inputs, id) in get_letter_inputs() {
        group.bench_with_input(BenchmarkId::new("Letter Product", id), &inputs, |b, i| {
            b.iter(|| letter_product(i.0, i.1))
        });
    }
    for (inputs, id) in get_word_inputs() {
        group.bench_with_input(BenchmarkId::new("Word Product", id), &inputs, |b, i| {
            b.iter(|| word_product(&i.0, &i.1))
        });
    }
    for (inputs, id) in get_braid_inputs() {
        group.bench_with_input(BenchmarkId::new("Braid Product", id), &inputs, |b, i| {
            b.iter(|| braid_product(&i.0, &i.1))
        });
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
