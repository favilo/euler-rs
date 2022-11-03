use std::{fmt::Debug, iter::Product, ops::Add};

use bit_set::BitSet;
use bit_vec::BitVec;
use itertools::Itertools;
use num::{range_inclusive, BigUint, Integer, One, Zero};

#[tracing::instrument]
pub fn fibonacci<T>() -> impl Iterator<Item = T>
where
    T: Add<T>,
    T: TryFrom<BigUint>,
    <T as TryFrom<num::BigUint>>::Error: Debug,
{
    itertools::unfold((BigUint::from(1u64), BigUint::from(1u64)), |(x1, x2)| {
        let next = x1.clone() + x2.clone();

        let ret = x1.clone();
        *x1 = x2.clone();
        *x2 = next;
        Some(ret.try_into().unwrap())
    })
}

#[tracing::instrument]
pub fn primes(max: usize) -> impl Iterator<Item = u64> {
    let set = {
        let mut set = BitSet::from_bit_vec(BitVec::from_elem(max, true));
        set.remove(0);
        set.remove(1);
        set
    };
    itertools::unfold(set, move |set| {
        let head = set.iter().next()?;
        (1..)
            .map(|n| n * head)
            .take_while(|&n| n < max)
            .for_each(|n| {
                set.remove(n);
            });
        Some(head as u64)
    })
}

pub fn is_palindrome(num: u64) -> bool {
    num.to_string() == num.to_string().chars().rev().collect::<String>()
}

pub fn digits(num: &BigUint) -> Vec<u64> {
    let s: String = num.to_string();
    s.chars().map(|c| c as u64 - '0' as u64).collect_vec()
}

pub fn triangles() -> impl Iterator<Item = u64> {
    (1..).map(|n| (n * (n + 1)) / 2)
}

// pub fn collatz_chain(n: u64) -> impl Iterator<Item = u64> {
//     iterate(n, |n: u64| if n.is_even() { n / 2 } else { 3 * n + 1 }).take_while(|n| n != 1)
// }

#[memoize::memoize]
pub fn collatz_count(n: u64) -> u64 {
    if n == 1 {
        return 1;
    }
    fn next_collatz(n: u64) -> u64 {
        if n.is_even() {
            n / 2
        } else {
            3 * n + 1
        }
    }
    1 + collatz_count(next_collatz(n))
}

pub fn factorial<N: Into<BigUint>>(num: N) -> BigUint {
    fn inner(n: BigUint) -> BigUint {
        if BigUint::zero() == n {
            BigUint::one()
        } else {
            BigUint::product(range_inclusive(BigUint::one(), n).into_iter())
        }
    }
    inner(num.into())
}
