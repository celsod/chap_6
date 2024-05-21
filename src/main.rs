fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"), //enum inside a struct
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"), //enum inside a struct
    };

    let home = IpAddrKind::V4(String::from("127.0.0.1")); //consider this a function that takes a string argument
    //and returns an instance of IpAddrKind type
    let loopback = IpAddrKind::V6(String::from("::1")); //enum with data directly inputted into it
}

enum IpAddrKind {
    V4(String), //enum now has an associated string value
    V6(String),
}

fn route(ip_kind: IpAddrKind){

}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message { //better than a structure because the enum includes various types within the single type
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Option<T> { //enum that can encode the conept of a value being present or absent.
    None,
    Some(T),
}