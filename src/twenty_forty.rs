use bit_set::BitSet;
use euler_macro::problem;
use itertools::{iproduct, Itertools};
use num::BigUint;
use tracing::info;

use crate::{
    common::{digits, fibonacci},
    utils::{
        divisors::FactorCache,
        numbers::{quadratic_prime_run_length, reciprocal_cycle_length},
        strings::{names, word_letter_score},
    },
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

#[problem(24)]
fn permutations() -> u64 {
    let mut perms = (0u32..10)
        .permutations(10)
        .map(|v| {
            v.iter()
                .map(|v| char::from_u32('0' as u32 + v).unwrap())
                .collect::<String>()
        })
        .collect_vec();
    perms.sort();

    perms[999_999].parse().unwrap()
}

#[problem(25)]
fn thousand_fibonacci() -> u64 {
    let large_digit = fibonacci::<BigUint>()
        .enumerate()
        .skip_while(|(_, f)| digits(&f).len() < 1000)
        .next()
        .unwrap();
    (large_digit.0 + 1) as u64
}

#[problem(26)]
fn reciprocal_cycles() -> u64 {
    (2..1000)
        .map(|n| (n, reciprocal_cycle_length(n)))
        .max_by_key(|t| t.1)
        .unwrap()
        .0 as u64
}

#[problem(27)]
fn quadratic_primes() -> i32 {
    let (a, b, _s) = iproduct!(-999..1000, -1000..=1000)
        .map(|(a, b)| (a, b, quadratic_prime_run_length(a, b)))
        .filter(|&(_a, _b, s)| s != 0)
        .max_by_key(|&(_, _, s)| s)
        .unwrap();
    a * b
}
