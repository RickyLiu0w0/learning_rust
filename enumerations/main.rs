enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
    V4_2(IpAddrV4),
    V6_2(IpAddrV6),
}

struct IpAddrV4 {
    // --snip--
    address: String,
}

struct IpAddrV6 {
    // --snip--
    address: String,
}

fn route(ip_kind: IpAddrKind) {}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    route(four);
    route(six);

    let home = IpAddrKind::V4_2(IpAddrV4 {
        address: String::from("127.0.0.1"),
    });

    let lockback = IpAddrKind::V6_2(IpAddrV6 {
        address: String::from("::1"),
    });
    route(home);
    route(lockback);
}
