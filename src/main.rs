// Copyright (c) 2024 Chaitanya Tata
// License: MIT
use clap::Parser;
use env_logger::Env;
use env_logger;
use inquire::Text;
use log;
use std::net::UdpSocket;
use std::time::Duration;
use std::str;
use hex;

const CONN_TIMEOUT_S: u64 = 5;

/// IP address and port of DUT
#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    ip: String,
    #[clap(short, long)]
    port: u16,
}

/// Connect and get a UdpSocket
fn connect_with_timeout(timeout_s: u64) -> std::io::Result<UdpSocket> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let timeout = Duration::from_secs(timeout_s);
    socket.set_read_timeout(Some(timeout))?;
    socket.set_write_timeout(Some(timeout))?;
    Ok(socket)
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = Cli::parse();
    println!("CA: {}: {}", args.ip, args.port);

    let socket = connect_with_timeout(CONN_TIMEOUT_S);
    match socket {
        Ok(_) => {
            println!("Connected to DUT");
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
            return;
        }
    }

    let socket = socket.unwrap();

    // Open an interactive prompt in a loop
    let cmd = Text::new("Enter hex string: ");
    // Send command to DUT, wait for resp, if exit break
    // TODO: Add ctrl-c support
    loop {
        let mut resp = [0; 1024];
        let cmd = cmd.clone().prompt().unwrap();
        if cmd == "exit" {
            break;
        }
        let cmd = hex::decode(cmd).unwrap();
        log::info!("Sending command: {:?}", cmd);
        let result = socket.send_to(&cmd, format!("{}:{}", args.ip, args.port));
        match result {
            Ok(_) => {
                log::info!("Sent command to DUT");
            }
            Err(e) => {
                log::error!("Failed to send command: {}", e);
                return;
            }
        }
        let (bytes_read, src) = match socket.recv_from(&mut resp) {
            Ok((bytes_read, src)) => (bytes_read, src),
            Err(e) => {
                log::error!("Failed to receive from socket: {}", e);
                return;
            }
        };
        log::info!("Received {} bytes from {}", bytes_read, src);

        let resp = match str::from_utf8(&resp[..bytes_read]) {
            Ok(v) => v,
            Err(e) => {
                log::error!("Failed to convert bytes to UTF-8: {}", e);
                return;
            }
        };
        log::info!("Response: {}", resp);
    }
}
