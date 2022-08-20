use crate::timer_future::TimerFuture;
use executor::new_executor_and_spawner;
use std::time::Duration;

mod executor;
mod timer_future;

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // Spawn a task to print before and after waiting on a timer.
    spawner.spawn(async {
        println!("hello!");
        // Wait for our timer future to complete after two seconds.
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    spawner.spawn(async {
        println!("hello again!");
        // Wait for our timer future to complete after two seconds.
        TimerFuture::new(Duration::new(1, 0)).await;
        println!("done again!");
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();
}
