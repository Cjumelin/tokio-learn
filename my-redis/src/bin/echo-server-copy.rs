use tokio::io::{self, AsyncRead};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();
            if io::copy(reader, writer).await.is_err() {
                println!("Fail to copy");
            }
        });
    }

    //let mut buf = [0; 2048];

    //while let Ok(n) = socket.read(buf.as_mut_slice()).await {
    //    if n <= 0 {
    //        break;
    //    }
    //    println!("Read bytes: {:?}", &buf[..n])
    //}
    //
    //println!("Nothing left to read");
    Ok(());
}
