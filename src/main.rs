use std::{
    io::Error,
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    process::Command,
    time::Duration,
};

use tokio::{
    task::spawn_blocking,
    time::{delay_for, timeout},
};

const TIMEOUT: Duration = Duration::from_secs(2);
const RANDOM_HOST_NAME: &str = "google.ca:80";

// Restart NAC (MacOS only).
fn restart_wifi() {
    Command::new("networksetup")
        .arg("-setairportpower")
        .arg("en0")
        .arg("off")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    Command::new("networksetup")
        .arg("-setairportpower")
        .arg("en0")
        .arg("on")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
}

// Resolve host -> IP.
// Runs blocking I/O inside a green thread so that I can
// time out if resolving the URL takes too long.
async fn resolve_host() -> Result<SocketAddr, Error> {
    let addr = spawn_blocking(move || {
        RANDOM_HOST_NAME
            .to_socket_addrs()
            .and_then(|mut iter| Ok(iter.next().unwrap()))
    })
    .await?;
    Ok(addr.unwrap())
}

// Check if establishing TCP connection is possible.
async fn test_online_status() {
    timeout(TIMEOUT, resolve_host())
        .await
        .and_then(|ip| {
            // Timeout has not occurred.
            let res =
                TcpStream::connect_timeout(&ip.unwrap(), TIMEOUT).or_else(|_| Err(restart_wifi()));
            Ok(res)
        })
        .or_else(|_| Err(restart_wifi()));
}

// DNS may be successful if cached, therefore, a successful resolution does not mean a valid internet connection.
// Try to establish TCP connection as backup.
#[tokio::main]
async fn main() {
    loop {
        test_online_status().await;
        delay_for(TIMEOUT).await;
    }
}
