use std::num::ParseFloatError;
use failure::Fail;

#[derive(Debug, Fail, Eq, PartialEq)]
pub enum MoneyError {
    #[fail(display = "Invalid input: {}", _0)]
    ParseAmount(ParseFloatError),

    #[fail(display = "{}", _0)]
    ParseFormatting(String),
}

fn parse_money(input: &str) -> Result<(f32, String), MoneyError> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts[..] {
        [amount, currency] => Ok((amount.parse()?, currency.to_string())),
        _ => Err(MoneyError::ParseFormatting(
            "Expecting amount and currency".into()
        )),
    }
}

impl From<ParseFloatError> for MoneyError {
    fn from(e: ParseFloatError) -> Self {
        MoneyError::ParseAmount(e)
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_money, MoneyError};

    #[test]
    fn parse_int_works() {
        let (money, unit) = parse_money("120 Euro").unwrap();
        assert_eq!(money, 120f32);
        assert_eq!("Euro", unit);
    }

    #[test]
    fn parse_float_works() {
        let (money, unit) = parse_money("140.01 Euro").unwrap();
        assert_eq!(money, 140.01f32);
        assert_eq!("Euro", unit);
    }

    #[test]
    fn parse_index_out_of_bounds() {
         let result = parse_money("140.01");
        assert_eq!(true, result.is_err());
        if let Some(e) = result.err() {
            assert_eq!(MoneyError::ParseFormatting("Expecting amount and currency".to_string()), e);
        }
    }
}
