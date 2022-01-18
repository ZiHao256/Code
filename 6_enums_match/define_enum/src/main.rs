enum IpAddrKind{
    IPv4,
    IPv6
}

struct IpAddr{
    kind: IpAddrKind,
    value: String
}


fn main() {
    let home = IpAddr{
        kind: IpAddrKind::IPv4,
        value: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: IpAddrKind::IPv6,
        value: String::from("::1"),
    };
    


}
