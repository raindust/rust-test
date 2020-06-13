fn parse_money(input: &str) -> (i32, String) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let maybe_amount = parts[0].parse();
    if maybe_amount.is_err() {
        return (-1, "invalid".to_string());
    }
    let currency = parts[1].to_string();
    return (maybe_amount.unwrap(), currency);
}


#[cfg(test)]
mod tests {
    use crate::parse_money;

    #[test]
    fn parse_int_works() {
        let (money, unit) = parse_money("120 Euro");
        assert_eq!(money, 120);
        assert_eq!("Euro", unit);
    }
}
