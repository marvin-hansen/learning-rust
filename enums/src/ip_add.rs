use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ipv4Addr {
    pub octets: [u8; 4],
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ipv6Addr {
    pub octets: [u8; 16],
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IpAddrKind {
    V4,
    // alternatively  V4(Ipv4Addr),
    V6, // V6(Ipv6Addr)
}

impl fmt::Display for IpAddrKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            IpAddrKind::V4 => write!(f, "IPv4"),
            IpAddrKind::V6 => write!(f, "IPv6"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IpAddr {
    addr: String,
    kind: IpAddrKind,
}

impl IpAddr {
    pub fn new(addr: String, kind: IpAddrKind) -> Self {
        IpAddr { addr, kind }
    }
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.addr, self.kind)
    }
}


