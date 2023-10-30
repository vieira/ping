pub mod icmp;
pub mod echo;
pub mod ip;

pub const ICMP_ECHO_REPLY: u8 = 0;
pub const ICMP_UNREACH: u8 = 3;
pub const ICMP_ECHO_REQUEST: u8 = 8;

pub const ICMP6_UNREACH: u8 = 1;
pub const ICMP6_ECHO_REQUEST: u8 = 128;
pub const ICMP6_ECHO_REPLY: u8 = 129;

