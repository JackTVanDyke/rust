use tokio::{net::TcpListener, io::{AsyncWriteExt, BufReader, AsyncBufReadExt}, sync::broadcast};

#[tokio::main]
async fn main() {
    let tcp_listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, rx) = broadcast::channel(10);

    loop {

        let (mut socket, addr) = tcp_listener.accept().await.unwrap();

        let tx = tx.clone();

        let mut rx = rx.resubscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            
            loop {
                tokio::select! {
                    res = reader.read_line(&mut line) => {
                        if res.unwrap() == 0 {
                            break;
                        }

                        tx.send((line.clone(), addr)).unwrap();
                        line.clear()
                    }
                    res = rx.recv() => {
                        let (msg, other_addr) = res.unwrap();

                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();    
                        }
                    }
                }
            }
        });
    }  
}
