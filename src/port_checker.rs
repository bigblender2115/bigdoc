use std::net::TcpListener;
use colored::Colorize;

pub fn check_ports(ports: Vec<u16>) {
    println!("------------- [PORTS]-------------");
    for port in ports {
        match TcpListener::bind(format!("127.0.0.1:{}", port)) {
            Ok(_) => println!("{:<12} {} is free", "[FREE]".green(), port),
            Err(_) => println!("{:<12} {} is in use", "[IN USE]".red(), port),
        }
    }
}