use async_executor::{Executor, Task};
use futures::{executor::block_on, future::join_all, Future};
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

lazy_static! {
    static ref EXECUTOR: Executor<'static> = Executor::new();
    static ref TASK_MAP: Mutex<HashMap<u64, Vec<Task<()>>>> = Mutex::new(HashMap::new());
}

fn main() {
    for seq_number in 0..10 {
        if seq_number % 4 == 0 || seq_number % 4 == 1 || seq_number % 4 == 2 {
            add_task(
                seq_number,
                EXECUTOR.spawn(async move { sync_handler(seq_number).await }),
            );
        } else {
            run(seq_number - 3);
            run(seq_number - 2);
            run(seq_number - 1);
        }
    }
}

fn run(seq_number: u64) {
    if let Some(tasks) = take_tasks(seq_number) {
        block_on(EXECUTOR.run(async { join_all(tasks).await }));
    }
}

fn take_tasks(seq_number: u64) -> Option<Vec<Task<()>>> {
    TASK_MAP.lock().unwrap().remove(&seq_number)
}

async fn sync_handler(seq_number: u64) {
    send_sync(seq_number, print_msg).await;

    if seq_number % 2 == 1 {
        async_handler(seq_number).await;
    }
}

async fn async_handler(seq_number: u64) {
    let msg = send_async(seq_number).await;
    print_msg(msg).await;
}

async fn send_sync<Handler, Fut>(seq_number: u64, handler: Handler)
where
    Handler: Fn(Result<String, String>) -> Fut + Sync + Send + 'static,
    Fut: Future<Output = ()> + Sync + Send + 'static,
{
    let task = EXECUTOR.spawn(async move {
        let msg = seq_number.to_string();
        handler(Ok(msg)).await;
    });
    task.await
}

async fn send_async(seq_number: u64) -> Result<String, String> {
    let task = EXECUTOR.spawn(async move {
        let msg = seq_number.to_string();
        // std::thread::sleep(Duration::from_secs(1));
        msg
    });

    Ok(task.await)
}

async fn print_msg(msg: Result<String, String>) {
    match msg {
        Ok(m) => println!("{}", m),
        Err(e) => println!("ERROR: {}", e),
    }
}

fn add_task(seq_number: u64, task: Task<()>) {
    let mut map = TASK_MAP.lock().unwrap();
    match map.get_mut(&seq_number) {
        Some(tasks) => tasks.push(task),
        None => {
            map.insert(seq_number, vec![task]);
        }
    }
}
