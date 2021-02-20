pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
