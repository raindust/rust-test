use futures::executor::block_on;

async fn hello_word() {
    println!("Hello, world!");
}

pub fn block_on_test() {
    let future = hello_word();
    block_on(future);
}