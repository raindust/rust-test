use std::thread;

fn main() {
    let v = vec![1, 3, 5];

    let handle = thread::spawn(move || {
        println!("this is a vector {:?}", v);
    });

    handle.join().unwrap();

    // compile error
    //println!("this is a vector {:?}", v);
}
