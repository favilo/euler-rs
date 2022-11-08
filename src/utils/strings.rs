use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, space0},
    combinator::{map, opt},
    error::VerboseError,
    multi::many1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

pub(crate) fn name(input: &str) -> IResult<&'_ str, String, VerboseError<&'_ str>> {
    map(delimited(tag("\""), alpha1, tag("\"")), ToOwned::to_owned)(input)
}

pub(crate) fn names(input: &str) -> IResult<&str, Vec<String>, VerboseError<&str>> {
    many1(terminated(name, opt(tuple((tag(","), space0)))))(input)
}

pub fn letter_score(c: char) -> usize {
    c as usize - 'A' as usize + 1
}

pub fn word_letter_score(s: &str) -> usize {
    s.chars().map(letter_score).sum()
}
