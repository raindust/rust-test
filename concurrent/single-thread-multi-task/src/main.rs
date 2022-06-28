use executor::{new_executor_and_spawner, Spawner};
use futures::executor::block_on;
use msg_future::{MsgFuture, MsgState, SharedState};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    task::Waker,
    time::Duration,
    vec,
};

pub mod executor;
pub mod msg_future;

type SharedStateMap = HashMap<u32, Vec<SharedState>>;

fn main() {
    let (executor, spawner) = new_executor_and_spawner();
    let mut shared_state_map: SharedStateMap = Default::default();

    for seq_number in 0..100 {
        if seq_number % 4 == 0 || seq_number % 4 == 1 || seq_number % 4 == 2 {
            block_on(async_handler(seq_number, &spawner, &mut shared_state_map));
        } else {
            run(seq_number - 3, seq_number, &executor, &mut shared_state_map);
            run(seq_number - 2, seq_number, &executor, &mut shared_state_map);
            run(seq_number - 1, seq_number, &executor, &mut shared_state_map);
        }
    }
}

fn run(
    key: u32,
    seq_number: u32,
    executor: &executor::Executor,
    shared_state_map: &mut SharedStateMap,
) {
    if let Some(states) = shared_state_map.remove(&key) {
        for state in states {
            if let Some(waker) = try_get_waker(key, seq_number, &state) {
                waker.wake();
                executor.run(key);
            } else {
                println!("failed to get waker of {}", seq_number - 1);
            }
        }
    } else {
        println!("failed to get waker of {}", seq_number - 1);
    }
}

fn try_get_waker(key: u32, seq_number: u32, shared_state: &Arc<Mutex<MsgState>>) -> Option<Waker> {
    let mut state = shared_state.lock().unwrap();
    state.msg = Some(format!("{}-{}", key, seq_number));
    state.waker.clone()
}

async fn async_handler(seq_number: u32, spawner: &Spawner, shared_state_map: &mut SharedStateMap) {
    let (future, state) = MsgFuture::new();
    let waker = send_async(seq_number, future, spawner);

    state.lock().unwrap().waker = Some(waker);
    add_state(seq_number, state, shared_state_map);

    if seq_number % 2 == 1 {
        another_async_handler(seq_number, spawner, shared_state_map).await;
    }
}

async fn another_async_handler(
    seq_number: u32,
    spawner: &Spawner,
    shared_state_map: &mut SharedStateMap,
) {
    let (future, state) = MsgFuture::new();
    let waker = send_async(seq_number, future, spawner);

    state.lock().unwrap().waker = Some(waker);
    add_state(seq_number, state, shared_state_map);
}

fn add_state(seq_number: u32, state: SharedState, shared_state_map: &mut SharedStateMap) {
    match shared_state_map.get_mut(&seq_number) {
        Some(states) => states.push(state),
        None => {
            shared_state_map.insert(seq_number, vec![state]);
        }
    }
}

fn send_async(seq_number: u32, future: MsgFuture, spawner: &Spawner) -> Waker {
    let waker = spawner.spawn(seq_number, async move {
        let msg = future.await;
        match msg {
            Ok(m) => println!("{}", m),
            Err(e) => println!("ERROR: {}", e),
        }
        wait_and_print().await;
    });

    waker
}

async fn wait_and_print() {
    std::thread::sleep(Duration::from_secs(1));
    println!("");
}
