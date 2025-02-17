// Using catch-all error types like `Box<dyn Error>` isn't recommended for
// library code where callers might want to make decisions based on the error
// content instead of printing it out or propagating it further. Here, we define
// a custom error type to make it possible for callers to decide what to do next
// when our function returns an error.

use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
enum CreationError {
    Negative,
    Zero,
}

// A custom error type that we will be using in `PositiveNonzeroInteger::parse`.
#[derive(PartialEq, Debug)]
#[allow(dead_code)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    #[allow(dead_code)]
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    fn from_parse_int(err: ParseIntError) -> ParsePosNonzeroError{
        ParsePosNonzeroError::ParseInt(err)
    }
    // TODO: Add another error conversion function here.
    // fn from_parseint(???) -> Self { ??? }
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    #[allow(dead_code)]
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }
    #[allow(dead_code)]
    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        // TODO: change this to return an appropriate error instead of panicking
        // when `parse()` returns an error.
        let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parse_int)?;
        // let x: i64 = s.parse().map_err(|e|ParsePosNonzeroError::from_parse_int(e))?;
        PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}
