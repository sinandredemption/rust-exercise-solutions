use std::collections::HashMap;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::{TcpListener, TcpStream, ToSocketAddrs, tcp::OwnedWriteHalf}, select, sync::{Notify, mpsc, oneshot}
};
use std::sync::Arc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type DisconnectData = (String, mpsc::UnboundedReceiver<String>);

enum BrokerEvent {
    NewMessage{from: String, to: Vec<String>, msg: String},
    NewPeer{name: String, writer: OwnedWriteHalf, client_disconnected: oneshot::Receiver<()>},
}

async fn broker_loop(mut rx: mpsc::UnboundedReceiver<BrokerEvent>)
{
    let mut hash_map: HashMap<String, mpsc::UnboundedSender<String>> = HashMap::new();
    let (disconnect_channel_tx, mut disconnect_channel_rx) = mpsc::unbounded_channel::<DisconnectData>();

    loop {
        select! {
            ev = rx.recv() => {
                match ev {
                    Some(event) => match event {
                        BrokerEvent::NewPeer { name, mut writer, client_disconnected} => {
                            println!("Peer {} joined!", name);
                            let (tx, mut rx) = mpsc::unbounded_channel::<String>();

                            hash_map.insert(name.clone(), tx);

                            let dctx = disconnect_channel_tx.clone();
                            tokio::spawn(async move {
                                if let Err(e) = connection_writer_loop(&mut rx, &mut writer, client_disconnected).await
                                {
                                    eprintln!("connection_writer_error: {e}");
                                }
                                let _ = dctx.send((name, rx));
                            });
                        },
                        
                        BrokerEvent::NewMessage { from, to, msg } => {
                            println!("Sending '{}'", msg);
                            for dest in to {
                                match hash_map.get(&dest) {
                                    Some(tx) => {
                                        if let Err(_) = tx.send(format!("from {}: {}\n", from, msg)) {
                                            println!("Failed sending message from {} to {}", from, dest);
                                        }
                                    },
                                    None => {println!("Route to destination {} not found", dest);},
                                };
                            }
                        },
                    },
                    None => {
                        println!("Goodbye from broker!");
                        break;
                    },
                }
            }
            f = disconnect_channel_rx.recv() => {
                match f {
                    Some((name, rx)) => {
                        hash_map.remove(&name);
                        std::mem::drop(rx);
                    },
                    None => {}
                }
            }
        }
    }

    println!("Draining pending channels on shutdown...");
    std::mem::drop(disconnect_channel_tx);
    while let Some((name, rx)) = disconnect_channel_rx.recv().await {
        println!("Dropping {}...", name);
        hash_map.remove(&name);
        std::mem::drop(rx);
    };
}

async fn connection_writer_loop(rx: &mut mpsc::UnboundedReceiver<String>, output: &mut OwnedWriteHalf, mut stop: oneshot::Receiver<()>) -> Result<()>
{
    loop {
        tokio::select! {
            msg = rx.recv() => {
                match msg {
                    Some(message) => output.write_all(message.as_bytes()).await?,
                    None => break,
                }
            }
            _ = &mut stop => {
                break
            }
        }
    }

    Ok(())
}

async fn connection_loop(stream: TcpStream, broker_tx: mpsc::UnboundedSender<BrokerEvent>, notify_on_exit: Arc<Notify>) -> Result<()> {
    
    let (reader, writer) = stream.into_split();
    let reader = BufReader::new(reader);

    let (_client_connected, client_disconnected) = tokio::sync::oneshot::channel::<()>(); // TODO Pick a better name later

    let mut lines = reader.lines();

    let user = tokio::select! {
        res = lines.next_line() => {
            match res {
                Ok(Some(line)) => { line.trim_end().to_string() },
                Ok(None) => { return Err("Premature Disconnection".into()); },
                Err(e) => { eprintln!("Error: {e}"); return Err(e.into()); },
            }
        },
        _ = notify_on_exit.notified() => { return Ok(()); },
    };

    broker_tx.send(BrokerEvent::NewPeer { name: user.clone(), writer, client_disconnected})?;

    loop {
        let s = tokio::select! {
            res = lines.next_line() => {
                match res {
                    Ok(Some(line)) => { line },
                    Ok(None) => { println!("User Logged out"); break Ok(()); },
                    Err(e) => { eprintln!("Error: {e}"); break Err(e.into()); },
                }
            },
            _ = notify_on_exit.notified() => { break Ok(()); },
        };

        let (dest, msg) = match s.find(':') {
            Some(idx) => {
                let (dest, msg) = s.split_at(idx);
                (dest.to_string(), msg[1..].trim().to_string())
            },
            None => {
                println!("Invalid message recieved"); continue;
            }
        };


        let dest: Vec<String> = dest.split(',').map(|d| d.trim().to_string()).collect();
        
        broker_tx.send(BrokerEvent::NewMessage { from: user.clone(), to: dest, msg: msg })?;

    }
}

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    // Start broker loop
    let (broker_tx, broker_rx) = mpsc::unbounded_channel::<BrokerEvent>();
    let broker_handle = tokio::task::spawn(async move {
        broker_loop(broker_rx).await;
    });

    let listener = TcpListener::bind(addr).await?;
    let notify_on_exit = Arc::new(Notify::new());
    loop { // Connection loop
        tokio::select! {
            res = listener.accept() => {
                let (stream, _) = res?;
                let broker_tx = broker_tx.clone();
                let notify_on_exit = Arc::clone(&notify_on_exit);
                tokio::task::spawn(async move {
                    match connection_loop(stream, broker_tx, notify_on_exit).await {
                        Ok(_) => {},
                        Err(e) => { eprintln!("{e}"); },
                    };
                });
            },
            res = tokio::signal::ctrl_c() => {
                res?;
                notify_on_exit.notify_waiters();
                std::mem::drop(broker_tx);
                broker_handle.await?;
                break Ok(())
            },
        }
    }
}

#[tokio::main]
pub(crate) async fn main() -> Result<()> {
    accept_loop("127.0.0.1:8080").await
}