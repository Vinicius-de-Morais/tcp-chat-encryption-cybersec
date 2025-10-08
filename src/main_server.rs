use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

type Clients = Arc<Mutex<Vec<TcpStream>>>;

fn handle_client(mut stream: TcpStream, clients: Clients) {
    let peer_addr = stream.peer_addr().unwrap();
    println!("Client connected: {}", peer_addr);

    let mut reader = BufReader::new(stream.try_clone().unwrap());

    loop {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected: {}", peer_addr);
                break;
            }
            Ok(_) => {
                println!("{} says: {}", peer_addr, buffer.trim());

                let clients = clients.lock().unwrap();
                for client in clients.iter() {
                    if client.peer_addr().unwrap() != peer_addr {
                        let _ = writeln!(client.try_clone().unwrap(), "{}", buffer);
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading from {}: {}", peer_addr, err);
                break;
            }
        }
    }

    // Remove client
    let mut clients = clients.lock().unwrap();
    clients.retain(|c| c.peer_addr().unwrap() != peer_addr);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on 127.0.0.1:8080");

    let clients: Clients = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        let stream = stream?;
        let clients = Arc::clone(&clients);

        clients.lock().unwrap().push(stream.try_clone().unwrap());

        thread::spawn(move || {
            handle_client(stream, clients);
        });
    }

    Ok(())
}
