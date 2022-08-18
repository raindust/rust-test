mod client;
mod server;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    const PORT: u16 = 12345;

    tokio::spawn(async move {
        if let Err(e) = server::serve(PORT).await {
            println!("Top server error: {}", e);
        }
    });

    client::client_main(PORT).await
}
