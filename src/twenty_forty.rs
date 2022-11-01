use euler_macro::problem;
use itertools::Itertools;
use tracing::info;

use crate::utils::divisors::FactorCache;

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
