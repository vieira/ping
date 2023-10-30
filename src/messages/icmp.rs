use super::{ICMP_UNREACH, ICMP6_UNREACH, ICMP6_ECHO_REQUEST};

#[derive(Debug)]
#[repr(C)]
pub struct Icmp<M> {
    pub r#type: u8,
    pub code: u8,
    pub checksum: u16,
    pub body: M,
}

impl<M> Icmp<M> {
    pub fn size(&self) -> usize {
        core::mem::size_of::<Self>()
    }

    pub fn as_bytes(&mut self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                self.size(),
            )
        }
    }

    pub fn checksum(&mut self) -> &mut Self {
        if self.r#type == ICMP6_ECHO_REQUEST { return self }

        self.checksum = 0;
        let payload = self.as_bytes();
        let mut sum = (1..payload.len()).step_by(2).fold(0, |acc, i| {
            acc + (u32::from(payload[i-1]) << 8) + u32::from(payload[i])
        });
        sum = (sum >> 16) + (sum & 0xffff);
        let checksum = !sum as u16;
        self.checksum = checksum.to_be();
        self
    }

    pub fn is_err(&self) -> bool {
        self.r#type == ICMP_UNREACH || self.r#type == ICMP6_UNREACH
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct IncomingIcmp<V, M> {
    pub ip_header: V,
    pub icmp: Icmp<M>,
}

impl<V, M> IncomingIcmp<V, M> {
    pub unsafe fn from_bytes(bytes: &[u8]) -> &Self {
        let strut: *const Self = bytes.as_ptr() as *const Self;
        &*strut
    }
}
