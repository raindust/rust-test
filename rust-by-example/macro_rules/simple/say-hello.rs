
macro_rules! say_hello {
    () => (
        // () indicates that the macro takes no argument.
        println!("Hello!");
    )
}

fn main() {
    say_hello!();
}
