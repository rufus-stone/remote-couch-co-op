use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

use simple_logger::SimpleLogger;
use structopt::StructOpt;

mod opt;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            if size > 0 {
                // Process msg
                let msg = std::str::from_utf8(&data).unwrap();
                log::info!("Got {} byte msg: {}", size, msg);
                if msg.contains("PING") {
                    log::info!("Replying with PONG...");
                    stream.write_all(b"PONG  ").unwrap();
                }
                true
            } else {
                false
            }
        }
        Err(_) => {
            log::error!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    // Get the CLI args
    let args = opt::Opt::from_args();

    // Set the logging level
    match &args.verbose {
        0 => SimpleLogger::new()
            .with_level(log::LevelFilter::Error)
            .init()
            .unwrap(),
        1 => SimpleLogger::new()
            .with_level(log::LevelFilter::Warn)
            .init()
            .unwrap(),
        2 => SimpleLogger::new()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap(),
        3 => SimpleLogger::new()
            .with_level(log::LevelFilter::Debug)
            .init()
            .unwrap(),
        _ => SimpleLogger::new()
            .with_level(log::LevelFilter::Trace)
            .init()
            .unwrap(),
    }

    let socket = &args.socket;
    let listener = TcpListener::bind(socket).unwrap();

    log::info!("Listening on: {:?}", socket);
    log::info!("Listening on: {:?}", listener);

    // Accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                log::info!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                log::error!("Error: {}", e);
            }
        }
    }

    // Stop listening
    drop(listener);
}
