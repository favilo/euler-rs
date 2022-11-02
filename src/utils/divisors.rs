use std::iter::once;

use itertools::Itertools;
use num::Integer;
use tracing::info;

use crate::common::primes;

pub(crate) struct FactorCache {
    primes: Vec<u64>,
    max_prime: usize,
}

impl FactorCache {
    pub(crate) fn new() -> Self {
        let max_prime = 1_000_000;
        let primes = primes(max_prime).collect();
        Self { primes, max_prime }
    }

    pub(crate) fn factors<'cache>(&'cache mut self, n: u64) -> Factors<'cache> {
        Factors {
            cache: self,
            curr: n,
            last_idx_checked: 0,
        }
    }

    pub(crate) fn prime(&mut self, idx: usize) -> u64 {
        if idx >= self.primes.len() {
            self.primes = primes(self.max_prime * 2).collect();
            self.max_prime *= 2;
        }
        self.primes[idx]
    }

    pub(crate) fn count_divisors(&mut self, n: u64) -> u64 {
        if n == 1 {
            return 1;
        }
        let factor_counts = self
            .factors(n)
            .dedup_with_count()
            .fold(1, |accum, c| accum * (c.0 + 1));
        factor_counts as u64 + 1
    }

    pub(crate) fn sum_proper_divisors(&mut self, n: u64) -> u64 {
        self.proper_divisors(n).sum()
    }

    pub(crate) fn proper_divisors(&mut self, n: u64) -> Box<dyn Iterator<Item = u64>> {
        if n == 1 {
            return Box::new(once(1));
        }
        let mut divisors = self
            .factors(n)
            .powerset()
            .map(|ps| if ps.is_empty() { vec![1] } else { ps })
            .map(|ps| ps.iter().product::<u64>())
            .filter(|&p| p != n)
            .collect_vec();
        divisors.sort();
        Box::new(divisors.into_iter().dedup())
    }
}

impl Default for FactorCache {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) struct Factors<'cache> {
    cache: &'cache mut FactorCache,
    curr: u64,
    last_idx_checked: usize,
}

impl Iterator for Factors<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr <= 1 {
            return None;
        }
        let mut candidate = self.cache.prime(self.last_idx_checked);
        while candidate <= self.curr {
            if self.curr.is_multiple_of(&candidate) {
                self.curr /= candidate;
                return Some(candidate);
            }
            self.last_idx_checked += 1;
            candidate = self.cache.prime(self.last_idx_checked);
        }

        None
    }
}
