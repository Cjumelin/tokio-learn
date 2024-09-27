use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:3000").await?;
    let (mut read, mut write) = socket.into_split();

    tokio::spawn(async move {
        write.write_all(b"Hello\r\n").await?;
        write.write_all(b"world\r\n").await?;
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0, 128];

    loop {
        let n = read.read(&mut buf).await?;

        if n == 0 {
            break;
        }
        println!("Read {:?} bytes\n read value = {:?}", &buf[..n], n);
    }

    println!("Nothing left to read");
    Ok(())
}
