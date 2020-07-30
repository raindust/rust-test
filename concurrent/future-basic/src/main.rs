mod block_on;
mod await_test;

use crate::block_on::block_on_test;
use self::await_test::await_test;

fn main() {
    block_on_test();
    await_test();
}
