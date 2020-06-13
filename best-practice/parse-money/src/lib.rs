use std::num::ParseFloatError;

#[derive(Debug)]
pub enum MoneyError {
   ParseError,
}

fn parse_money(input: &str) -> Result<(f32, String), MoneyError> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        Err(MoneyError::ParseError)
    } else {
        let (amount, currency) = (parts[0], parts[1]);
        Ok((amount.parse()?, currency.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_money;

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
        let (money, unit) = parse_money("140.01").unwrap();
    }
}
