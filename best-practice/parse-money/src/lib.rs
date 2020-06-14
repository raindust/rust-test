use std::num::ParseFloatError;
use failure::Fail;

#[derive(Debug, Eq, PartialEq)]
pub enum Currency {
    Dollar,
    Euro,
}

#[derive(Debug)]
pub struct Money {
    amount: f32,
    currency: Currency,
}

#[derive(Debug, Fail, Eq, PartialEq)]
pub enum MoneyError {
    #[fail(display = "Invalid input: {}", _0)]
    ParseAmount(ParseFloatError),

    #[fail(display = "{}", _0)]
    ParseFormatting(String),

    #[fail(display = "{}", _0)]
    ParseCurrency(String),
}

impl std::str::FromStr for Currency {
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "dollar" | "$" => Ok(Currency::Dollar),
            "euro" | "eur" | "€" => Ok(Currency::Euro),
            _ => Err(MoneyError::ParseCurrency("Unknown currency".into())),
        }
    }
}

impl Money {
    fn new(amount: f32, currency: Currency) -> Self {
        Money { amount, currency }
    }
}

impl std::str::FromStr for Money {
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        match parts[..] {
            [amount, currency] => Ok(Money::new(amount.parse()?, currency.parse()?)),
            _ => Err(MoneyError::ParseFormatting(
                "Expecting amount and currency".into()
            )),
        }
    }
}

impl From<ParseFloatError> for MoneyError {
    fn from(e: ParseFloatError) -> Self {
        MoneyError::ParseAmount(e)
    }
}

#[cfg(test)]
mod tests {
    use crate::{MoneyError, Money, Currency};

    #[test]
    fn parse_int_works() {
        let money = "120 Euro".parse::<Money>().unwrap();
        assert_eq!(120f32, money.amount);
        assert_eq!(Currency::Euro, money.currency);
    }

    #[test]
    fn parse_float_works() {
        let money = "140.01 $".parse::<Money>().unwrap();
        assert_eq!(140.01f32, money.amount);
        assert_eq!(Currency::Dollar, money.currency);
    }

    #[test]
    fn parse_index_out_of_bounds() {
        let result = "140.01".parse::<Money>();
        assert_eq!(true, result.is_err());
        if let Some(e) = result.err() {
            assert_eq!(MoneyError::ParseFormatting("Expecting amount and currency".to_string()), e);
        }
    }
}
