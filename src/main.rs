use std::{
    net::{SocketAddr, TcpStream},
    process::Command,
    thread,
    time::Duration,
};

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

fn main() {
    let addr = SocketAddr::from(([172, 217, 14, 195], 80));
    loop {
        if let Err(_) = TcpStream::connect_timeout(&addr, Duration::from_secs(2)) {
            restart_wifi();
        }

        thread::sleep(Duration::new(3, 0));
    }
}
