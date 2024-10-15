use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 128];

            let n = socket.read(buf.as_mut_slice()).await;

            match n {
                Ok(0) => {
                    println!("EOF");
                    return;
                }
                Ok(n) => {
                    if socket.write(&buf[..n]).await.is_err() {
                        println!("Error writing back to socket");
                        return;
                    }
                }
                Err(e) => {
                    println!("Error reading the socket: {:?}", e);
                    return;
                }
            }
        });
    }
}
