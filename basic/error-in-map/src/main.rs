use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let strings = vec!["93", "tofu", "18"];
    let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("Results: {:?}", numbers);

    let a: Option<&str> = Some("tofu");
    let b: Result<Option<i32>, _> = a.map(|a| a.parse::<i32>()).transpose();
    println!("Results: {:?}", b);

    let a: Option<&str> = Some("33");
    let b: Result<Option<i32>, _> = a.map(|a| a.parse::<i32>()).transpose();
    println!("Results: {:?}", b);

    Ok(())
}
