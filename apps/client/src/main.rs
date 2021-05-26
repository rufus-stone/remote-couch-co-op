use simple_logger::SimpleLogger;
use structopt::StructOpt;

use std::io::{Read, Write};
use std::net::TcpStream;

mod opt;

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

    // Which console platform are we emulating?
    let platform = &args.platform;
    log::info!("{:?}", platform);

    // What is the server IP/port?
    let server = &args.server;
    log::info!("{:?}", server);

    // Connect to the server
    match TcpStream::connect(server) {
        Ok(mut stream) => {
            log::info!("Connection established!");

            let start = std::time::Instant::now();

            let msg = b"PING";
            stream.write_all(msg).unwrap();

            let mut data = [0; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let reply = std::str::from_utf8(&data).unwrap();
                    log::info!("Reply: {}", reply);
                }
                Err(e) => {
                    log::error!("Failed to receive data: {}", e);
                }
            }

            let latency = std::time::Instant::now().duration_since(start);
            log::info!("Latency: {} microseconds", latency.as_micros());
        }
        Err(e) => {
            log::error!("Failed to connect: {}", e);
        }
    }
}
