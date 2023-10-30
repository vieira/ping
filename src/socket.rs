use std::io::Result;
use std::net::SocketAddr;
use std::time::Duration;
use socket2::{SockAddr, Socket, Domain, Type, Protocol};

pub struct IcmpSocket(Socket);

impl IcmpSocket {
    pub fn bind(addr: &str) -> Option<IcmpSocket> {
        let local: SocketAddr = format!("{addr}:0").parse().ok()?;
        let socket = match local {
            SocketAddr::V4(_) => Socket::new(
                Domain::IPV4,
                Type::RAW,
                Some(Protocol::ICMPV4),
            ),
            SocketAddr::V6(_) => Socket::new(
                Domain::IPV6,
                Type::RAW,
                Some(Protocol::ICMPV6),
            ),
        }.ok()?;
        socket.bind(&local.into()).ok()?;
        let _ = socket.set_read_timeout(Some(Duration::new(2, 0)));
        Some(IcmpSocket(socket))
    }

    pub fn recv_from(&self) -> Result<(Vec<u8>, SocketAddr)> {
        let mut buffer = Vec::with_capacity(4096);
        let (len, addr) = self.0.recv_from(buffer.spare_capacity_mut())?;
        unsafe { buffer.set_len(len) };
        Ok((buffer, addr.as_socket().unwrap()))
    }

    pub fn send_to(&self, buf: &[u8], addr: &SockAddr) -> Result<usize> {
        self.0.send_to(buf, addr)
    }
}
