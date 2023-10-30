mod messages;
mod resolver;
mod socket;

use std::{process, thread, env};
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use messages::{ICMP_ECHO_REQUEST, ICMP6_ECHO_REQUEST};
use messages::ip::{Ipv4Header, Ipv6Header};
use messages::icmp::{Icmp, IncomingIcmp};
use messages::echo::{Echo, Data};

fn main() {
    let pid = Box::leak(Box::new(process::id() as u16));
    let epoch = Box::leak(Box::new(Instant::now()));

    let Some(domain) = env::args().nth(1)
    else {
        eprintln!("usage: ping <host|mcast-group>");
        process::exit(64);
    };

    let Some(dest) = resolver::lookup_host(&domain)
    else {
        eprintln!("ping: cannot resolve {domain}");
        process::exit(68);
    };

    let local = match dest {
        SocketAddr::V4(_) => "0.0.0.0",
        SocketAddr::V6(_) => "[::]",
    };

    let Some(socket) = Box::leak(Box::new(socket::IcmpSocket::bind(local)))
    else {
        eprintln!("ping: failed to bind socket");
        process::exit(77);
    };

    let r#type = match dest {
        SocketAddr::V4(_) => ICMP_ECHO_REQUEST,
        SocketAddr::V6(_) => ICMP6_ECHO_REQUEST,
    };

    let mut req = Icmp::<Echo>::new(r#type, *pid);
    println!(
        "PING {domain} ({}): {} data bytes",
        dest.ip(),
        core::mem::size_of::<Data>()
    );

    thread::spawn(|| {
        loop {
            let Ok((buffer, src_addr)) = socket.recv_from() else {
                eprintln!(
                    "Request timeout ({}s into execution)",
                    epoch.elapsed().as_secs(),
                );
                continue
            };

            let (icmp, ttl) = match src_addr {
                SocketAddr::V4(_) => unsafe {
                    let reply = IncomingIcmp::<Ipv4Header, Echo>::from_bytes(
                        &buffer,
                    );
                    (&reply.icmp, Some(reply.ip_header.ttl))
                },
                SocketAddr::V6(_) => unsafe {
                    let reply = IncomingIcmp::<Ipv6Header, Echo>::from_bytes(
                        &buffer,
                    );
                    (&reply.icmp, None)
                },
            };

            if !icmp.is_ours(*pid) { continue }

            if icmp.is_err() {
                eprintln!(
                    "Destination unreachable: src={} code={} ({}s into execution)",
                    src_addr.ip(),
                    icmp.code,
                    epoch.elapsed().as_secs(),
                );
            }

            let now = epoch.elapsed().as_micros() as u64;
            let timestamp = u64::from_be_bytes(icmp.body.data.timestamp);
            let ttl = match ttl {
                Some(ttl) => format!(" ttl={}", ttl),
                None => String::new(),
            };
            println!(
                "{} bytes from {}: icmp_seq={}{} time={} ms",
                icmp.size(),
                src_addr.ip(),
                icmp.body.seq_no,
                ttl,
                (now - timestamp) as f64 / 1000.0,
            );
        }
    });

    for seq_no in 0.. {
        let payload = req.timestamp(&epoch).checksum().as_bytes();
        let _ = socket.send_to(payload, &dest.into());
        thread::sleep(Duration::new(1, 0));
        req.body.seq_no = (seq_no % u16::MAX) + 1;
    }
}
