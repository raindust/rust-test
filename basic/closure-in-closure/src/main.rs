fn fun_a<F>(a: i32, cal: F) where F: Fn(i32) -> i32 {
    let print = || {
       let res = cal(a);
        println!("{}", res);
    };

    print();
}

fn main() {
    fun_a(3, |a| a * 2);
}
