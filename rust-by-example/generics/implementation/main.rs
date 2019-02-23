struct Val {
    val: f64
}

struct GetVal<T> {
    get_val: T
}

impl Val {
    fn value(&self) -> &f64 { &self.val }
}

impl GetVal<i32> {
    fn value_i(&self) -> &i32 { println!("i'm int"); &self.get_val }
}

impl <T> GetVal<T> {
    fn value(&self) -> &T { &self.get_val }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GetVal { get_val: 4.0 };
    let z = GetVal { get_val: 5 };

    println!("{}, {}, {}", x.value(), y.value(), z.value_i());
}
