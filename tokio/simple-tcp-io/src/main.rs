use tokio::{
    io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    const PORT: u16 = 12345;

    tokio::spawn(async move {
        if let Err(e) = serve(PORT).await {
            println!("Top server error: {}", e);
        }
    });

    let mut stdin = io::stdin();
    let mut buf = vec![0; 1024];
    while (stdin.read(&mut buf).await).is_ok() {
        let line = String::from_utf8(buf.clone()).unwrap();
        println!("stdin: {}", line);
        tokio::spawn(async move {
            if let Err(e) = client_request(PORT, &line).await {
                println!("Top client error: {}", e);
            }
        });
    }
    Ok(())
}

async fn client_request(port: u16, message: &str) -> io::Result<()> {
    let stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;

    let (rd, wr) = io::split(stream);

    let mut writer = BufWriter::new(wr);
    writer.write(message.as_bytes()).await?;
    writer.flush().await?;

    let mut reader = BufReader::new(rd);
    let mut buffer = String::new();
    reader.read_line(&mut buffer).await?;

    println!("client received: {}", buffer);

    Ok(())
}

async fn serve(port: u16) -> io::Result<()> {
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
