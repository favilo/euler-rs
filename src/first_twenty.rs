use color_eyre::eyre::eyre;
// use num::integer::sqrt;

use crate::common::{fibonacci, primes};

pub(crate) fn run(problem: u32) -> color_eyre::Result<u64> {
    Ok(match problem {
        1 => problem_1(),
        2 => problem_2(),
        3 => problem_3(),
        _ => Err(eyre!("Not defined yet"))?,
    })
}

fn problem_1() -> u64 {
    let threes = (1..)
        .map(|i| i * 3)
        .filter(|n| n % 5 != 0)
        .take_while(|&n| n < 1000);
    let fives = (1..).map(|i| i * 5).take_while(|&n| n < 1000);
    threes.chain(fives).sum()
}

fn problem_2() -> u64 {
    fibonacci()
        .take_while(|&f| f < 4_000_000)
        .filter(|&f| f & 1 == 0)
        .sum()
}

fn problem_3() -> u64 {
    let check = 600851475143;
    let max = 775146;
    let prime = primes(10).collect::<Vec<_>>();
    tracing::info!("{prime:?}");
    let primes = primes(max).collect::<Vec<_>>();
    primes
        .into_iter()
        .rev()
        .filter(|p| check % p == 0)
        .next()
        .unwrap()
}
