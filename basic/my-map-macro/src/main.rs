#[macro_export]
macro_rules! my_map {
    ($($k:expr => $v:expr), + $(,)?) => ({
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert($k, $v);
        )*
        map
    });
}

fn main() {
    let map = my_map! {
        "a" => 1,
        "b" => 2,
        "c" => 3,
    };
    println!("{map:?}");

    let map = my_map! {
        "d" => 4,
        "e" => 5,
        "f" => 6
    };
    println!("{map:?}");
}
