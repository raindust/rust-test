use executor::{new_executor_and_spawner, Spawner};
use msg_future::{MsgFuture, MsgState};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    task::Waker,
};

pub mod executor;
pub mod msg_future;

fn main() {
    let (executor, spawner) = new_executor_and_spawner();
    let mut shared_state_map: HashMap<u32, Arc<Mutex<MsgState>>> = Default::default();

    for seq_number in 0..100 {
        if seq_number % 4 == 0 || seq_number % 4 == 1 {
            let (future, state) = MsgFuture::new(seq_number);

            let waker = send_async(seq_number, future, &spawner);

            state.lock().unwrap().waker = Some(waker);
            shared_state_map.insert(seq_number, state);
        } else {
            let keys: Vec<u32> = shared_state_map.keys().map(|v| *v).collect();

            for num in keys {
                if let Some(state) = shared_state_map.remove(&num) {
                    if let Some(waker) = try_get_waker(num, seq_number, &state) {
                        waker.wake();
                        executor.run(num);
                    } else {
                        println!("failed to get waker of {}", seq_number - 1);
                    }
                } else {
                    println!("failed to get waker of {}", seq_number - 1);
                }
            }
        }
    }
}

fn try_get_waker(key: u32, seq_number: u32, shared_state: &Arc<Mutex<MsgState>>) -> Option<Waker> {
    let mut state = shared_state.lock().unwrap();
    state.msg = Some(format!("{}-{}", key, seq_number));
    state.waker.clone()
}

fn send_async(seq_number: u32, future: MsgFuture, spawner: &Spawner) -> Waker {
    let waker = spawner.spawn(seq_number, async move {
        let msg = future.await;
        match msg {
            Ok(m) => println!("{}", m),
            Err(e) => println!("ERROR: {}", e),
        }
    });

    waker
}
