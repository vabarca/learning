
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener, sync::broadcast};

/*

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4545").await.unwrap();

    loop{
        let (mut socket, _addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            
            loop {
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                if bytes_read == 0{
                    break;
                }
                print!("{line}");
                writer.write_all(line.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
 }

 */

 /*

 #[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4545").await.unwrap();
    let (tx, _rx) = broadcast::channel(100);

    loop{
        let (mut socket, _addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            
            loop {
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                if bytes_read == 0{
                    break;
                }
                tx.send(line.clone()).unwrap();

                let msg = rx.recv().await.unwrap();
                writer.write_all(msg.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
 }

*/

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4545").await.unwrap();
    let (tx, _rx) = broadcast::channel(100);

    loop{
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    _ = reader.read_line(&mut line) => {
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        match result{
                            Ok((msg, msg_addr)) => {
                                if addr != msg_addr{
                                    writer.write_all(msg.as_bytes()).await.unwrap()
                                }
                            },
                            Err(e) => println!("{}", e)
                        }
                    }
                }
            }
        });
    }
 }