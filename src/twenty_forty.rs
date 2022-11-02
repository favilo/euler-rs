use std::collections::HashSet;

use bit_set::BitSet;
use euler_macro::problem;
use itertools::Itertools;
use tracing::info;

use crate::utils::{
    divisors::FactorCache,
    strings::{names, word_letter_score},
};

#[problem(21)]
fn problem() -> u64 {
    let mut cache = FactorCache::new();
    info!(
        "{}, {}",
        cache.sum_proper_divisors(3),
        cache.sum_proper_divisors(4),
    );
    (1..10_000)
        .filter_map(|n| {
            let d = cache.sum_proper_divisors(n);
            (d != n && d < 10_000 && cache.sum_proper_divisors(d) == n).then_some(n)
        })
        .sum()
}

#[problem(22)]
fn problem() -> u64 {
    let (input, mut names) =
        names(include_str!("../data/p022_names.txt")).expect("Should parse clean");
    assert_eq!(input, "");
    names.sort();
    names
        .iter()
        .map(|s| word_letter_score(s))
        .enumerate()
        .map(|(i, score)| ((i + 1) * score) as u64)
        .sum()
}

#[problem(23)]
fn problem() -> u64 {
    let mut cache = FactorCache::new();
    let abundant_numbers: BitSet<usize> = (1usize..)
        .filter(|n| cache.sum_proper_divisors(*n as u64) > *n as u64)
        .take_while(|n| n < &28123)
        .collect();
    (1..28123)
        .filter(|n| {
            !abundant_numbers
                .iter()
                .any(|a| abundant_numbers.contains(n - a))
        })
        .sum::<usize>() as u64
}
