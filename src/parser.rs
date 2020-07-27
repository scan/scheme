use nom::{
    alt, char, delimited, many0, map, named, none_of, number::complete::double, one_of, IResult,
};

use crate::value::LispValue;

#[allow(dead_code)]
fn vec_to_str(vec: Vec<char>) -> String {
    vec.iter().collect()
}

named!(symbol<&str, char>, one_of!("!#$%&|*+-/:<=>?@^_~"));
named!(string<&str, String>, map!(delimited!(char!('"'), many0!(none_of!("\"")), char!('"')), vec_to_str));

named!(parse_string<&str, LispValue>, map!(string, LispValue::String));
named!(parse_number<&str, LispValue>, map!(double, LispValue::Number));

named!(parse_expr<&str, LispValue>, alt!(parse_string | parse_number));

pub fn parse(input: &str) -> IResult<&str, LispValue> {
    parse_string(input)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_string() {
        assert_eq!(
            parse_string("\"hello world\""),
            Ok(("", LispValue::String("hello world".to_owned())))
        );
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("3.1415"), Ok(("", LispValue::Number(3.1415))));
    }
}
