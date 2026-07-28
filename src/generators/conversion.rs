use crate::{ArtinGenerator, BandGenerator, Sign, Strand};
use std::collections::BTreeSet;

pub fn artin_to_band(generators: &[ArtinGenerator]) -> Vec<BandGenerator> {
    let mut generators_remaining = generators;
    let mut bands = Vec::new();

    while !generators_remaining.is_empty() {
        let (band, advance_by) = collect_generators(generators_remaining);
        bands.push(band);
        generators_remaining = &generators_remaining[advance_by..];
    }

    bands
}

fn collect_generators(generators: &[ArtinGenerator]) -> (BandGenerator, usize) {
    let mut pivot = if generators.len().is_multiple_of(2) {
        generators.len() / 2
    } else {
        generators.len().div_ceil(2)
    };

    while pivot > 1 {
        let crossing = generators[pivot];
        let mut ll_staircase = BTreeSet::new();
        let mut lu_staircase = BTreeSet::new();
        let mut rl_staircase = BTreeSet::new();
        let mut ru_staircase = BTreeSet::new();
        let mut look = 1;

        while look < pivot {
            let left = generators[pivot - look];
            let right = generators[pivot + look];

            match left.sign() {
                Sign::Positive => {
                    if left.foot() <= crossing.foot() || lu_staircase.contains(&left.foot()) {
                        break;
                    }
                    lu_staircase.insert(left.foot());
                }
                Sign::Negative => {
                    if left.foot() >= crossing.foot() || ll_staircase.contains(&left.foot()) {
                        break;
                    }
                    ll_staircase.insert(left.foot());
                }
            }
            match right.sign() {
                Sign::Positive => {
                    if right.foot() >= crossing.foot() || rl_staircase.contains(&right.foot()) {
                        break;
                    }
                    rl_staircase.insert(right.foot());
                }
                Sign::Negative => {
                    if right.foot() <= crossing.foot() || ru_staircase.contains(&right.foot()) {
                        break;
                    }
                    ru_staircase.insert(right.foot());
                }
            }

            look += 1;
        }

        if look == pivot {
            return (
                BandGenerator::new(
                    *ll_staircase.iter().min().unwrap(),
                    *lu_staircase.iter().min().unwrap(),
                    crossing.sign(),
                )
                .unwrap(),
                2 * pivot + 1,
            );
        } else {
            pivot -= 1;
        }
    }

    (
        BandGenerator::new(
            Strand::new(1).unwrap(),
            Strand::new(2).unwrap(),
            generators[pivot].sign(),
        )
        .unwrap(),
        1,
    )
}

pub fn band_to_artin(bands: &[BandGenerator]) -> Vec<ArtinGenerator> {
    bands
        .iter()
        .fold(Vec::new(), |w, b| [w, decompose_band(b)].concat())
}

fn decompose_band(band: &BandGenerator) -> Vec<ArtinGenerator> {
    // Band decomposition is infallible, so it's safe to unwrap any intermediate results
    let crossing = ArtinGenerator::new((band.head() - 1).unwrap(), band.sign()).unwrap();
    let mut left = Vec::new();
    for foot_idx in band.foot().index()..(band.head().index() - 1) {
        left.push(ArtinGenerator::new(Strand::new(foot_idx).unwrap(), Sign::Negative).unwrap());
    }
    let right = left.iter().rev().map(|a| -*a).collect();
    [left, vec![crossing], right].concat()
}
