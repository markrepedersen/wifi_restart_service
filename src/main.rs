use std::{
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    process::Command,
    thread,
    time::Duration,
};

const URL: &str = "http://detectportal.firefox.com/success.txt";

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

fn is_online(addr: SocketAddr) -> Result<TcpStream, std::io::Error> {
    let timeout = Duration::new(1, 0);
    TcpStream::connect_timeout(&addr, timeout)
}

fn resolve_dns() -> Result<SocketAddr, std::io::Error> {
    let mut socket_addrs = URL.to_socket_addrs()?;
    Ok(socket_addrs.next().unwrap())
}

fn main() {
    loop {
        if let Err(_) = resolve_dns().and_then(is_online) {
            restart_wifi();
        }
        thread::sleep(Duration::new(3, 0));
    }
}
