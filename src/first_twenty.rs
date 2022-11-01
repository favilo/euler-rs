use std::{cmp, str::FromStr};

use chrono::{Datelike, Weekday};

use itertools::Itertools;
use ndarray::indices;
use num::{BigInt, BigUint};

use euler_macro::problem;
use once_cell::sync::Lazy;
use primes::{PrimeSet, Sieve};
// use num::integer::sqrt;

use crate::{
    common::{collatz_count, digits, factorial, fibonacci, is_palindrome, triangles},
    utils::{divisors::FactorCache, numbers::number_to_words, parse_grid, parse_triangle},
};

#[problem(1)]
fn problem() -> u64 {
    let threes = (1..)
        .map(|i| i * 3)
        .filter(|n| n % 5 != 0)
        .take_while(|&n| n < 1000);
    let fives = (1..).map(|i| i * 5).take_while(|&n| n < 1000);
    threes.chain(fives).sum()
}

#[problem(2)]
fn problem_2() -> u64 {
    fibonacci()
        .take_while(|&f| f < 4_000_000)
        .filter(|&f| f & 1 == 0)
        .sum()
}

#[problem(3)]
fn problem_3() -> u64 {
    let check = 600851475143;
    let max = 775146;
    let mut sieve = Sieve::new();
    let primes = sieve.iter();
    let primes = primes.take_while(|p| *p < max).collect::<Vec<_>>();
    primes
        .into_iter()
        .rev()
        .filter(|p| check % p == 0)
        .next()
        .unwrap()
}

#[problem(4)]
fn problem_4() -> u64 {
    itertools::iproduct!((100..1000).rev(), (100..1000).rev())
        .map(|(x, y)| x * y)
        .filter(|&n| is_palindrome(n))
        .max()
        .unwrap()
}

#[problem(5)]
fn problem_5() -> u64 {
    (1..=20).fold(1, |x, y| num::integer::lcm(x, y))
}

#[problem(6)]
fn problem_6() -> u64 {
    let squares: u64 = (1..=100).map(|n| n * n).sum();
    let sums: u64 = (1..=100).sum();
    let sums = sums * sums;
    sums - squares
}

#[problem(7)]
fn problem_7() -> u64 {
    let mut sieve = Sieve::new();

    sieve.get(10_000)
}

#[problem(8)]
fn problem_8() -> u64 {
    let digits = concat!(
        "73167176531330624919225119674426574742355349194934969835203127",
        "7450632623957831801698480186947885184385861560789112949495459501737958",
        "331952853208805511125406987471585238630507156932909632952274430435576689664895",
        "044524452316173185640309871112172238311362229893423380308135336276614282806444",
        "486645238749303589072962904915604407723907138105158593079608667017242712188399",
        "879790879227492190169972088809377665727333001053367881220235421809751254540594",
        "752243525849077116705560136048395864467063244157221553975369781797784617406495",
        "514929086256932197846862248283972241375657056057490261407972968652414535100474",
        "821663704844031998900088952434506585412275886668811642717147992444292823086346",
        "567481391912316282458617866458359124566529476545682848912883142607690042242190",
        "226710556263211111093705442175069416589604080719840385096245544436298123098787",
        "992724428490918884580156166097919133875499200524063689912560717606058861164671",
        "0940507754100225698315520005593572972571636269561882670428252483600823257530420752963450"
    )
    .chars()
    .map(|c| c as u64 - '0' as u64)
    .collect::<Vec<_>>();
    digits
        .array_windows::<13>()
        .map(|a| a.into_iter().copied().product())
        .max()
        .unwrap_or(0)
}

#[problem(9)]
fn problem_9() -> u64 {
    for a in 1..998 {
        for b in a + 1..1000 {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }
    unreachable!()
}

#[problem(10)]
fn problem_10() -> u64 {
    let mut p = Sieve::new();
    p.iter().take_while(|p| p < &2_000_000).sum()
}

#[problem(11)]
fn problem_11() -> u64 {
    let grid = textwrap::dedent(
        "\
        08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
        49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
        81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
        52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
        22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
        24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
        32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
        67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
        24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
        21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
        78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
        16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
        86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
        19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
        04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
        88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
        04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
        20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
        20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
        01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
        ",
    );
    let (_, grid) = parse_grid::<u64>(&grid).unwrap();
    let (width, height) = grid.dim();
    let horiz_product = grid
        .windows((4, 1))
        .into_iter()
        .map(|window| window.product())
        .max()
        .unwrap();
    let vert_product = grid
        .windows((1, 4))
        .into_iter()
        .map(|window| window.product())
        .max()
        .unwrap();
    let diag_product = indices((width - 4, height - 4))
        .into_iter()
        .map(|(x, y)| (0..4).map(|i| grid[(x + i, y + i)]).product::<u64>())
        .max()
        .unwrap();
    let diag2_product = indices((width - 4, height - 4))
        .into_iter()
        .map(|(row, col)| (row, col + 4))
        .map(|(row, col)| (0..4).map(|i| grid[(row + i, col - i)]).product::<u64>())
        .max()
        .unwrap();
    [horiz_product, vert_product, diag_product, diag2_product]
        .into_iter()
        .max()
        .unwrap()
}

#[problem(12)]
fn problem_12() -> u64 {
    let mut cache = FactorCache::new();
    triangles()
        .map(|n| (n, cache.count_divisors(n)))
        .find_map(|(n, count)| (count > 500).then_some(n))
        .unwrap()
}

#[problem(13)]
fn problem_13() -> u64 {
    let sum: BigInt = include_str!("../data/problem13.txt")
        .lines()
        .map(|l| BigInt::from_str(l).unwrap())
        .sum();
    sum.to_string()[..10].parse::<u64>().unwrap()
}

#[problem(14)]
fn problem_14() -> u64 {
    (1..1_000_000)
        .map(|n| (n, collatz_count(n)))
        .max_by(|x, y| x.1.cmp(&y.1))
        .unwrap()
        .0
}

#[problem(15)]
fn problem_15() -> u64 {
    let factorial_20 = factorial(20u64);
    (factorial(40u64) / (&factorial_20 * &factorial_20))
        .try_into()
        .unwrap()
}

#[problem(16)]
fn problem_16() -> u64 {
    digits(BigUint::from(2u32).pow(1000)).into_iter().sum()
}

#[problem(17)]
fn problem_17() -> u64 {
    (1..=1000)
        .map(|n| number_to_words(n).len() as u64)
        .sum::<u64>()
}

#[problem(18)]
fn problem_18() -> u64 {
    let input = "\
        75
        95 64
        17 47 82
        18 35 87 10
        20 04 82 47 65
        19 01 23 75 03 34
        88 02 77 73 07 63 67
        99 65 04 28 06 16 70 92
        41 41 26 56 83 40 80 70 33
        41 48 72 33 47 32 37 16 94 29
        53 71 44 65 25 43 91 52 97 51 14
        70 11 33 28 77 73 17 78 39 68 17 57
        91 71 52 38 17 14 91 43 58 50 27 29 48
        63 66 04 68 89 53 67 30 73 16 69 87 40 31
        04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
    ";
    let (_, rows) = parse_triangle::<u64>(input).unwrap();
    let mut sums: Vec<u64> = vec![];
    for row in rows.into_iter().rev() {
        let these_sums = if !sums.is_empty() {
            assert_eq!(sums.len(), row.len());
            // row.iter().for_each(|v| print!("{v:4} "));
            // println!();
            sums.into_iter().zip(&row).map(|(a, &b)| a + b).collect()
        } else {
            row
        };
        if these_sums.len() > 1 {
            sums = these_sums
                .iter()
                .tuple_windows()
                .map(|(a, b)| *cmp::max(a, b))
                .collect::<Vec<_>>();
            // sums.iter().for_each(|v| print!("{v:4} "));
        } else {
            sums = these_sums;
        }
        // println!();
    }
    sums[0]
}

#[problem(19)]
fn problem_19() -> u64 {
    (1901..=2000)
        .map(|year| (1..=12).map(move |month| chrono::NaiveDate::from_ymd(year, month, 1)))
        .flatten()
        .filter(|date| date.weekday() == Weekday::Sun)
        .count()
        .try_into()
        .unwrap()
}

#[problem(20)]
fn problem_20() -> u64 {
    digits(factorial(BigUint::from(100u64))).iter().sum()
}
