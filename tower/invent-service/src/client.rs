use tokio::{
    io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpStream,
};

pub(crate) async fn client_main(port: u16) -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buf = vec![0; 1024];
    while (stdin.read(&mut buf).await).is_ok() {
        let line = String::from_utf8(buf.clone()).unwrap();
        println!("stdin: {}", line);
        tokio::spawn(async move {
            if let Err(e) = client_request(port, &line).await {
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
