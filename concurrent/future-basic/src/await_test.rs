use futures::executor::block_on;
use futures::Future;
use futures::task::{Context, Poll};
use std::pin::Pin;

struct Song(String);

async fn learn_song() -> Song {
    let name = "rain rain goaway";
    println!("learning {}", name);
    Song(name.to_string())
}

async fn sing_song(song: Song) {
   println!("singing {}", song.0);
}

async fn dance() {
    println!("i am dancing");
}

pub fn await_test() {
    // block_on_mode();
    block_on(async_mode());
}

// fn block_on_mode() {
//     let song = block_on(learn_song());
//     block_on(song);
//     dance();
// }

async fn async_mode() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    // sing_song(song);  // 这里如果不加await就不会触发方法内部的println
    sing_song(song).await;
}

