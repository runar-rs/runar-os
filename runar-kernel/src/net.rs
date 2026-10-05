

/// A MAC_48 hardware address.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MacAddress([u8;6]);

pub struct EthernetFrame<'a> {
    pub destination: MacAddress,
    pub source: MacAddress,
    pub ethernet_type: EthernetType,
    pub data: &'a [u8],
}


#[repr(u16)]
pub enum EthernetType {
    Ipv4 = 0x0800,
    Arp = 0x0806,
    Ipv6 = 0x86dd,
}