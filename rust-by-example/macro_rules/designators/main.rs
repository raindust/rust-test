macro_rules! create_functions {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($func_name:ident) => (
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    )
}

// create functions
create_functions!(foo);
create_functions!(bar);

macro_rules! print_result{
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($expression:expr) => (
        println!("{:?} = {:?}", 
                 stringify!($expression),
                 $expression);
    )
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    print_result!({
        let x = 1u32;
        x*x+2*x-1
    })
}
