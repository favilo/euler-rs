use std::str::FromStr;

use bit_set::BitSet;
use bit_vec::BitVec;
use ndarray::Array2;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{digit1, newline},
    combinator::{map, map_res},
    multi::{many1, many_till},
    sequence::{pair, terminated},
    IResult,
};

#[tracing::instrument]
pub fn fibonacci() -> impl Iterator<Item = u64> {
    itertools::unfold((1, 2), |(x1, x2)| {
        let next = *x1 + *x2;

        let ret = *x1;
        *x1 = *x2;
        *x2 = next;
        Some(ret)
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

fn value<T: FromStr>(input: &str) -> IResult<&str, T> {
    let (input, v) = map_res(terminated(digit1, tag(" ")), |v: &str| v.parse())(input)?;
    Ok((input, v))
}

fn row<T: FromStr>(input: &str) -> IResult<&str, Vec<T>> {
    map(terminated(many_till(value, newline), newline), |v: ()| v)(input)
    // map_res(many_till(pair(take(2), tag(" ")), newline), |row| {
    //     row.into_iter().map(|v| T::try_from(v).unwrap())
    // })(input)
}

pub fn parse_grid<T: FromStr + Copy>(input: &str) -> IResult<&str, Array2<T>> {
    let (input, rows) = many1(row)(input)?;
    let height = rows.len();
    let width = rows[0].len();
    let values = rows.into_iter().flatten().collect::<Vec<_>>();

    Ok((
        input,
        Array2::from_shape_fn((width, height), |(i, j)| values[i * width + j]),
    ))
}
