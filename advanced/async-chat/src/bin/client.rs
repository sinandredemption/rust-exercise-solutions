

use std::ops::Add;

use tokio::io::{AsyncBufReadExt, BufReader, BufWriter, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::select;
use tokio::runtime::Runtime;

fn main() {
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async move {
        let stream = match TcpStream::connect("127.0.0.1:8080").await {
            Ok(stream) => stream,
            Err(_) => { println!("Couldn't connect to 127.0.0.1:8080"); return; },
        };

        let (reader, writer) = stream.into_split();

        let stdout_task = tokio::task::spawn(async {
            let reader = BufReader::new(reader);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                println!("{}", line);
            };
        });

        let stdin_task = tokio::task::spawn(async move {
            let mut writer = BufWriter::new(writer);
            let reader = BufReader::new(tokio::io::stdin());
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = writer.write_all(line.add("\n").as_bytes()).await;
                let _ = writer.flush().await;
            }
        });

        select! {
            _ = stdout_task => {}
            _ = stdin_task => {}
        };
    });

    runtime.shutdown_background();
}