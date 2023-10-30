use std::time::Instant;
use crate::messages::{ICMP_ECHO_REPLY, ICMP6_ECHO_REPLY};
use super::icmp::Icmp;

#[derive(Debug)]
#[repr(C)]
pub struct Data {
    pub timestamp: [u8;8],
}

#[derive(Debug)]
#[repr(C)]
pub struct Echo {
    identifier: u16,
    pub seq_no: u16,
    pub data: Data,
}

impl Icmp<Echo> {
    pub fn new(r#type: u8, identifier: u16) -> Self {
        Self {
            r#type,
            code: 0,
            checksum: 0,
            body: Echo {
                identifier: identifier.to_be(),
                seq_no: 0,
                data: Data { timestamp: [0;8] },
            }
        }
    }

    pub fn timestamp(&mut self, epoch: &Instant) -> &mut Self {
        let timestamp = epoch.elapsed().as_micros() as u64;
        self.body.data.timestamp = timestamp.to_be_bytes();
        self
    }

    pub fn is_ours(&self, identifier: u16) -> bool {
        (self.r#type == ICMP_ECHO_REPLY || self.r#type == ICMP6_ECHO_REPLY)
            && u16::from_be(self.body.identifier) == identifier
    }
}
