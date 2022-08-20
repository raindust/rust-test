use executor::{new_executor_and_spawner, Spawner};
use futures::{executor::block_on, Future};
use msg_future::{MsgFuture, MsgState, SharedState};
use std::{
    collections::HashMap,
    sync::{mpsc::sync_channel, Arc, Mutex},
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
            block_on(sync_handler(seq_number, &spawner, &mut shared_state_map));
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

async fn sync_handler(seq_number: u32, spawner: &Spawner, shared_state_map: &mut SharedStateMap) {
    send_sync(seq_number, spawner, shared_state_map, print_msg);

    if seq_number % 2 == 1 {
        send_sync(seq_number, spawner, shared_state_map, print_msg);
        // replace `send_sync` with `async_handler` will dead lock
        // async_handler(seq_number, spawner, shared_state_map).await;
    }
}

async fn async_handler(seq_number: u32, spawner: &Spawner, shared_state_map: &mut SharedStateMap) {
    let msg = send_async(seq_number, spawner, shared_state_map).await;
    print_msg(msg).await;
}

fn add_state(seq_number: u32, state: SharedState, shared_state_map: &mut SharedStateMap) {
    match shared_state_map.get_mut(&seq_number) {
        Some(states) => states.push(state),
        None => {
            shared_state_map.insert(seq_number, vec![state]);
        }
    }
}

async fn print_msg(msg: Result<String, String>) {
    match msg {
        Ok(m) => println!("{}", m),
        Err(e) => println!("ERROR: {}", e),
    }
}

fn send_sync<Handler, Fut>(
    seq_number: u32,
    spawner: &Spawner,
    shared_state_map: &mut SharedStateMap,
    handler: Handler,
) where
    Handler: Fn(Result<String, String>) -> Fut + Sync + Send + 'static,
    Fut: Future<Output = ()> + Sync + Send + 'static,
{
    let (future, state) = MsgFuture::new();

    let waker = spawner.spawn(seq_number, async move {
        let msg = future.await;
        handler(msg).await;
        wait_and_print().await;
    });

    state.lock().unwrap().waker = Some(waker);
    add_state(seq_number, state, shared_state_map);
}

async fn send_async(
    seq_number: u32,
    spawner: &Spawner,
    shared_state_map: &mut SharedStateMap,
) -> Result<String, String> {
    let (future, state) = MsgFuture::new();

    let (sender, receiver) = sync_channel(1);
    let waker = spawner.spawn(seq_number, async move {
        let msg = future.await;
        wait_and_print().await;
        sender.send(msg).expect("");
    });

    state.lock().unwrap().waker = Some(waker);
    add_state(seq_number, state, shared_state_map);

    receiver.recv().map_err(|e| e.to_string())?
}

async fn wait_and_print() {
    std::thread::sleep(Duration::from_secs(1));
    println!("");
}
