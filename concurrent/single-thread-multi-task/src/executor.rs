use futures::{
    future::BoxFuture,
    task::{waker_ref, ArcWake},
    Future, FutureExt,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    task::{Context, Waker},
};

pub type ExecutorContainer = Arc<Mutex<HashMap<u32, Vec<Arc<Task>>>>>;
pub type SpawnerContainer = Arc<Mutex<HashMap<u32, Vec<Arc<Task>>>>>;

/// Task executor that receives tasks off of a channel and runs them.
pub struct Executor {
    ready_queue: ExecutorContainer,
}

/// `Spawner` spawns new futures onto the task channel.
#[derive(Clone)]
pub struct Spawner {
    task_sender: SpawnerContainer,
}

/// A future that can reschedule itself to be polled by an `Executor`.
pub struct Task {
    seq_number: u32,
    /// In-progress future that should be pushed to completion.
    ///
    /// The `Mutex` is not necessary for correctness, since we only have
    /// one thread executing tasks at once. However, Rust isn't smart
    /// enough to know that `future` is only mutated from one thread,
    /// so we need to use the `Mutex` to prove thread-safety. A production
    /// executor would not need this, and could use `UnsafeCell` instead.
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// Handle to place the task itself back onto the task queue.
    task_sender: SpawnerContainer,
}

pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    // Maximum number of tasks to allow queueing in the channel at once.
    // This is just to make `sync_channel` happy, and wouldn't be present in
    // a real executor.
    //const MAX_QUEUED_TASKS: usize = 200;
    //let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    let container = Arc::new(Mutex::new(HashMap::new()));
    (
        Executor {
            ready_queue: container.clone(),
        },
        Spawner {
            task_sender: container,
        },
    )
}

impl Spawner {
    pub fn spawn(
        &self,
        seq_number: u32,
        future: impl Future<Output = ()> + 'static + Send,
    ) -> Waker {
        let future = future.boxed();
        let task = Arc::new(Task {
            seq_number,
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        let waker = waker_ref(&task).clone();
        self.task_sender
            .lock()
            .unwrap()
            .insert(seq_number, vec![task]);

        waker
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Implement `wake` by sending this task back onto the task channel
        // so that it will be polled again by the executor.
        let cloned = arc_self.clone();
        let mut task_sender = arc_self.task_sender.lock().unwrap();
        if let Some(container) = task_sender.get_mut(&cloned.seq_number) {
            container.push(cloned);
        } else {
            task_sender.insert(cloned.seq_number, vec![cloned]);
        }
    }
}

impl Executor {
    pub fn run(&self, seq_number: u32) {
        if let Some(tasks) = self.take_tasks(seq_number) {
            let mut pending_tasks = vec![];
            for task in tasks {
                // Take the future, and if it has not yet completed (is still Some),
                // poll it in an attempt to complete it.
                let mut future_slot = task.future.lock().unwrap();
                if let Some(mut future) = future_slot.take() {
                    // Create a `LocalWaker` from the task itself
                    let waker = waker_ref(&task);
                    let context = &mut Context::from_waker(&*waker);
                    // `BoxFuture<T>` is a type alias for
                    // `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.
                    // We can get a `Pin<&mut dyn Future + Send + 'static>`
                    // from it by calling the `Pin::as_mut` method.
                    if future.as_mut().poll(context).is_pending() {
                        // We're not done processing the future, so put it
                        // back in its task to be run again in the future.
                        *future_slot = Some(future);
                        pending_tasks.push(task.clone());
                    }
                }
            }

            if pending_tasks.is_empty() {
                self.push_tasks(seq_number, pending_tasks);
            }
            self.remove_timeout(seq_number);
        }
    }

    fn take_tasks(&self, seq_number: u32) -> Option<Vec<Arc<Task>>> {
        let mut queue = self.ready_queue.lock().unwrap();
        // println!("@@ queue length: {}", queue.len());
        queue.remove(&seq_number)
    }

    fn remove_timeout(&self, seq_number: u32) {
        self.ready_queue
            .lock()
            .unwrap()
            .retain(|k, _| k + 20 > seq_number);
    }

    fn push_tasks(&self, seq_number: u32, tasks: Vec<Arc<Task>>) {
        self.ready_queue.lock().unwrap().insert(seq_number, tasks);
    }
}
