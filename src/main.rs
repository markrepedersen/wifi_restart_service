use std::{net::TcpStream, process::Command, thread, time::Duration};

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
    loop {
        if let Err(e) = TcpStream::connect("rustlang.org:80") {
            println!("{}", e);
            restart_wifi();
        }

        thread::sleep(Duration::new(3, 0));
    }
}
