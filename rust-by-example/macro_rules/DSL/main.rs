macro_rules! calculate {
    (eval $e:expr) => {{
        {
            let var: usize = $e;
            println!("{} = {}", stringify!($e), var);
        }
    }};
}

fn main() {
    calculate! {
        eval 1 + 2
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}
