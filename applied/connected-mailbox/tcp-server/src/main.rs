use std::{
    collections::VecDeque, format, io::{self, BufRead, BufReader, BufWriter, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread,
};

fn handle_client(stream: TcpStream, deq: Arc<Mutex<VecDeque<String>>>) -> Result<(), io::Error> {
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
        let result = simple_db::parse(&line);
        match result {
            Ok(simple_db::Command::Publish(s)) => {
                writer.write_all(s.as_bytes())?;
                deq.lock().unwrap().push_back(s);
            }
            Ok(simple_db::Command::Retrieve) => {
                let s = deq
                    .lock()
                    .unwrap()
                    .pop_back()
                    .ok_or(io::ErrorKind::NotFound)?;
                writer.write_all(s.as_bytes())?;
            }
            Err(e) => {
                writer.write_all(format!("Err: {:?}", e).as_bytes())?;
            }
        };

        writer.flush()?;
        line.clear();
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let deq: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));

    thread::scope(|s| {
        for stream in listener.incoming() {
            // let deq_ = deq.clone();
            //std::thread::spawn(move || handle_client(stream.unwrap(), deq_).unwrap());
            s.spawn(|| handle_client(stream.unwrap(), deq.clone()).unwrap());
        }
    });

    Ok(())
}
