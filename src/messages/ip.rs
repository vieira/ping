
#[derive(Debug)]
#[repr(C)]
pub struct Ipv4Header {
    verihl: u8,
    tos: u8,
    tot_len: u16,
    id: u16,
    frag: u16,
    pub ttl: u8,
    protocol: u8,
    check: u16,
    saddr: [u8; 4],
    daddr: [u8; 4],
}

#[derive(Debug)]
pub struct Ipv6Header {}
