enum IpAddrKind{
    IPv4((u8, u8, u8, u8)),
    IPv6(String)
}

fn main() {
    let home = IpAddrKind::IPv4((127, 0, 0, 1));
    let loopback = IpAddrKind::IPv6(String::from("::1"));

    
}