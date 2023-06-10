enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum ForTheTut {
    V4(u8,u8,u8,u8),
    V6(String),
}

fn route(ip_kind: IpAddrKind){}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = ForTheTut::V4(127,0,0,1);
    let loopback2 = ForTheTut::V6(String::from("::1"));

}