use std::net::{SocketAddr, ToSocketAddrs};

pub fn lookup_host(host: &str) -> Option<SocketAddr> {
    format!("{host}:0").to_socket_addrs().ok()?.next()
}
