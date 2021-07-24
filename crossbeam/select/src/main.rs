use crossbeam_channel::{bounded, select, Receiver, RecvError, Sender, TryRecvError};

fn main() {
    // let (sender, receiver): (Sender<i32>, Receiver<i32>) = crossbeam_channel::bounded(3);
    let (sender, receiver): (Sender<i32>, Receiver<i32>) = crossbeam_channel::unbounded();

    let s1 = sender.clone();
    std::thread::spawn(move || {
        if let Err(e) = s1.send(6) {
            println!("send error: {}", e);
        }
    });

    println!("{:?}", receiver.recv());
    println!("{:?}", receiver.recv());
    // std::thread::sleep(std::time::Duration::from_secs(3));
    // loop {
    //     select! {
    //         recv(receiver) -> data => {
    //             match data {
    //                 Ok(n) => println!("got data: {:?}", n),
    //                 Err(e) => println!("receive failed: {}", e),
    //             }
    //         }
    //     }
    // }
}

// fn main() {
//     let (s, r) = bounded(1);
//
//     // Send a message into the channel.
//     s.send("foo").unwrap();
//
//     // This call would block because the channel is full.
//     // s.send("bar").unwrap();
//
//     // Receive the message.
//     assert_eq!(r.recv(), Ok("foo"));
//
//     // This call would block because the channel is empty.
//     println!("1");
//     r.recv();
//     println!("2");
//
//     // Try receiving a message without blocking.
//     // assert_eq!(r.try_recv(), Err(TryRecvError::Empty));
//
//     // Disconnect the channel.
//     // drop(s);
//
//     // This call doesn't block because the channel is now disconnected.
//     // assert_eq!(r.recv(), Err(RecvError));
// }
