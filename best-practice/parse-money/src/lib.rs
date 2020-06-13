fn parse_money(input: &str) -> (i32, String) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let maybe_amount = parts[0].parse().unwrap();
    let currency = parts[1].to_string();
    return (maybe_amount, currency);
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
