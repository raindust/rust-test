use executor::new_executor_and_spawner;
use msg_future::{MsgFuture, MsgState};
use std::{
    sync::{Arc, Mutex},
    task::Waker,
};

pub mod executor;
pub mod msg_future;

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    let mut shared_state: Arc<Mutex<MsgState>> = Default::default();
    for seq_number in 0..100 {
        if seq_number % 2 == 0 {
            let (future, state) = MsgFuture::new(seq_number);

            let waker = spawner.spawn(async {
                let msg = future.await;
                match msg {
                    Ok(m) => println!("{}", m),
                    Err(e) => println!("ERROR: {}", e),
                }
            });
            executor.run();

            shared_state = state;
            shared_state.lock().unwrap().waker = Some(waker);
        } else {
            if let Some(waker) = try_get_waker(seq_number, &shared_state) {
                waker.wake();
                executor.run();
            } else {
                println!("failed to get waker of {}", seq_number - 1);
            }
        }
    }
}

fn try_get_waker(seq_number: u32, shared_state: &Arc<Mutex<MsgState>>) -> Option<Waker> {
    let mut state = shared_state.lock().unwrap();
    state.msg = Some(format!("{}", seq_number));
    state.waker.clone()
}
