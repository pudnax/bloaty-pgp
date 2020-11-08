use nom::bytes::complete::take_while;
use nom::bytes::complete::take_while_m_n;
use nom::character::complete::newline;
use nom::IResult;

use super::utils::fold_into_string;

const BASE64_LINE_LENGTH: usize = 64_usize;

fn is_base64_digit(c: char) -> bool {
    (c >= '0' && c <= '9')
        || (c >= 'A' && c <= 'Z')
        || (c >= 'a' && c <= 'z')
        || c == '+'
        || c == '/'
        || c == '='
}

pub fn parse_base64(input: &str) -> IResult<&str, String> {
    let (input, mut base64) = fold_into_string(input, parse_base64_line)?;

    if input.starts_with('=') {
        return Ok((input, base64));
    }

    let (input, remaining) = take_while(is_base64_digit)(input)?;
    let (input, _) = newline(input)?;

    base64.push_str(remaining);

    Ok((input, base64))
}

/// Parse a single line of length BASE64_LINE_LENGTH which contains only base64 characters.
/// (and does not begin with an '='.)
fn parse_base64_line(input: &str) -> IResult<&str, &str> {
    if input.chars().next() == Some('=') {
        return Err(nom::Err::Error((input, nom::error::ErrorKind::Char)));
    }

    let (input, res) =
        take_while_m_n(BASE64_LINE_LENGTH, BASE64_LINE_LENGTH, is_base64_digit)(input)?;
    let (input, _) = newline(input)?;

    Ok((input, res))
}
