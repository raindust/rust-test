use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("one".to_string());
    });

    tokio::spawn(async {
        let _ = tx2.send("two".to_string());
    });

    loop {
        tokio::select! {
            val = rx1 => {
                println!("rx1 completed first with {:?}", val);
            }
            val = rx2 => {
                println!("rx2 completed first with {:?}", val);
            }
        }
    }
}
