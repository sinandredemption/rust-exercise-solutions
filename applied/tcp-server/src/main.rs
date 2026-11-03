use std::{io::{self, BufRead, BufReader, BufWriter, Write}, net::{TcpListener, TcpStream}, sync::mpsc::Sender};
use std::sync::mpsc;
fn handle_client(stream: TcpStream, tx: Sender<usize>) -> Result<(), io::Error> {
   // todo!("read stream line by line, write lines back to the stream");

    // for line in stream {
    //   write line back to the to stream
    // }
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream);
    let mut line = String::new();
    loop {
        let b = reader.read_line(&mut line)?;
        if b == 0 {
            break;
        }
        //log.lock().unwrap().push(line.len());
        //log.push(line.len());
        tx.send(b).unwrap();
        writer.write(line.as_bytes())?;
        writer.flush()?;
        line.clear();
    }
        

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    //let log: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(vec![]));
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let mut log: Vec<usize> = vec![];
        for recieved in rx {
            log.push(recieved);
        }
    });

    for stream in listener.incoming() {
        let tx = tx.clone();
        std::thread::spawn(move || { handle_client(stream.unwrap(), tx).unwrap() } );
    }
    
    Ok(())
}