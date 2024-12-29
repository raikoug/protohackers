use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn main() -> std::io::Result<()> {
    // Bind to TCP port 7 on all interfaces
    let listener = TcpListener::bind("0.0.0.0:7000")?;
    println!("Echo server listening on TCP port 7000...");

    // Accept connections in a loop
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(mut stream) => {
                // Spawn a thread to handle each connection concurrently
                thread::spawn(move || {
                    let mut buf = [0u8; 1024];
                    loop {
                        // Read data from the client
                        match stream.read(&mut buf) {
                            Ok(0) => {
                                // 0 bytes read indicates the client closed the connection
                                break;
                            }
                            Ok(n) => {
                                // Echo the received data back to the client
                                if let Err(e) = stream.write_all(&buf[..n]) {
                                    eprintln!("Failed to write to client: {}", e);
                                    break;
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to read from client: {}", e);
                                break;
                            }
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}
