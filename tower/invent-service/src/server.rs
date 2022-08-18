use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
};

pub(crate) async fn serve(port: u16) -> io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(e) = response_request(socket).await {
                println!("Server error: {}", e);
            }
        });
    }
}

async fn response_request(socket: TcpStream) -> io::Result<()> {
    let (rs, ws) = io::split(socket);

    let mut reader = BufReader::new(rs);
    let mut buffer = String::new();
    reader.read_line(&mut buffer).await?;
    println!("server read recieved: {}", buffer);

    let mut writer = BufWriter::new(ws);
    writer.write(b"response from server").await?;
    writer.flush().await?;

    Ok(())
}
