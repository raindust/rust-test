

trait MyTrait {
    type T;

    fn get(&self, i: u8) -> T;
}

struct A {
    value: u8
}

struct B {

}

impl MyTrait for B {
    type T = A;

    fn get(&self, i: u8) -> T {
        A {value: i}
    }
}


#[cfg(test)]
mod tests {
    use crate::{B, A, MyTrait};

    #[test]
    fn it_works() {
        let b = B {};
        let a = b.get(5) as A;
        let a2 = A {value: 5};

        assert_eq!( <B as MyTrait>::T::<A>.value, a.value);
    }
}
