use std::str::FromStr;

use ndarray::Array2;
use nom::{
    character::complete::{digit1, newline, space0},
    combinator::map_res,
    error::VerboseError,
    multi::{many1, many_till},
    sequence::delimited,
    IResult,
};

pub mod dates;
pub mod divisors;
pub mod numbers;
pub mod strings;

fn value<T: FromStr>(input: &str) -> IResult<&str, T, VerboseError<&str>> {
    let (input, v) = map_res(delimited(space0, digit1, space0), |v: &str| v.parse())(input)?;
    Ok((input, v))
}

fn row<T: FromStr>(input: &str) -> IResult<&str, Vec<T>, VerboseError<&str>> {
    let (input, (row, _)) = many_till(value, newline)(input)?;
    Ok((input, row))
}

pub fn parse_grid<T: FromStr + Copy>(input: &str) -> IResult<&str, Array2<T>, VerboseError<&str>> {
    let (input, rows) = many1(row)(input)?;
    let height = rows.len();
    let width = rows[0].len();
    let values = rows.into_iter().flatten().collect::<Vec<_>>();

    Ok((
        input,
        Array2::from_shape_fn((width, height), |(i, j)| values[i * height + j]),
    ))
}

pub fn parse_triangle<T: FromStr + Copy>(
    input: &str,
) -> IResult<&str, Vec<Vec<T>>, VerboseError<&str>> {
    many1(row)(input)
}
