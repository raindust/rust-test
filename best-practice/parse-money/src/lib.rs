use std::num::ParseFloatError;

fn parse_money(input: &str) -> Result<(f32, String), ParseFloatError> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let maybe_amount = parts[0].parse()?;
    let currency = parts[1].to_string();
    return Ok((maybe_amount, currency));
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
}
