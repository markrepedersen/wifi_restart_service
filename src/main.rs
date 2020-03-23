use std::{
    io::Error,
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    process::Command,
    thread,
    time::Duration,
};

use tokio::time::timeout;

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
async fn resolve_host() -> Result<SocketAddr, Error> {
    let mut addrs = RANDOM_HOST_NAME.to_socket_addrs()?;
    Ok(addrs.next().unwrap())
}

// Check if can establish TCP connection to machine.
async fn test_online_status() -> Result<(), Error> {
    if let Ok(ip) = timeout(TIMEOUT, resolve_host()).await? {
        TcpStream::connect_timeout(&ip, TIMEOUT)
            .or_else(|_| Err(restart_wifi()))
            .unwrap();
    } else {
        restart_wifi();
    }
    Ok(())
}

// DNS may be successful if cached, therefore, a successful resolution does not mean a valid internet connection.
// Try to establish TCP connection as backup.
#[tokio::main]
async fn main() {
    loop {
        test_online_status().await.unwrap();
        thread::sleep(TIMEOUT);
    }
}
