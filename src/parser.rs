use nom::{
    alt, char,
    character::{is_alphabetic, is_alphanumeric},
    delimited, many0, map, named, none_of,
    number::complete::double,
    one_of, tag, value, IResult, pair, take_while, take_while_m_n
};

use crate::value::LispValue;

#[allow(dead_code)]
fn vec_to_str(vec: Vec<char>) -> String {
    vec.iter().collect()
}

#[allow(dead_code)]
fn tuple_to_atom((c, v): (char, Vec<char>)) -> LispValue {
    LispValue::Atom(c.to_string() + v.into_iter().collect::<String>().as_str())
}

named!(symbol<&str, char>, one_of!("!#$%&|*+-/:<=>?@^_~"));
named!(alphasymbol<&str, char>, alt!(symbol | one_of!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")));
named!(alphanumsymbol<&str, char>, alt!(symbol | one_of!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")));

named!(string<&str, String>, map!(delimited!(char!('"'), many0!(none_of!("\"")), char!('"')), vec_to_str));

named!(parse_string<&str, LispValue>, map!(string, LispValue::String));
named!(parse_number<&str, LispValue>, map!(double, LispValue::Number));
named!(parse_true<&str, LispValue>, value!(LispValue::Bool(true), tag!("#t")));
named!(parse_false<&str, LispValue>, value!(LispValue::Bool(false), tag!("#f")));
named!(parse_atom<&str, LispValue>, map!(pair!(alphasymbol, many0!(alphanumsymbol)), tuple_to_atom));

named!(parse_expr<&str, LispValue>, alt!(parse_true | parse_false | parse_string | parse_number));

pub fn parse(input: &str) -> IResult<&str, LispValue> {
    parse_expr(input)
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn test_parse_true() {
        assert_eq!(parse_true("#t"), Ok(("", LispValue::Bool(true))))
    }

    #[test]
    fn test_parse_false() {
        assert_eq!(parse_false("#f"), Ok(("", LispValue::Bool(false))))
    }

    #[test]
    fn test_parse_atom() {
        assert_eq!(parse_atom("=="), Ok(("", LispValue::String("==".to_owned()))))
    }
}
