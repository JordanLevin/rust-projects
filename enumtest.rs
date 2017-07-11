enum IpAddressKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(ip_type: IpAddrKind){}

fn main(){
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1");
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

enum IpAddrOther {
    V4(String),
    V6(String),
}

let home = IpAddrOther::V4(String::from("127.0.0.1"));
let loopbac = IpAddrOther::V6(String::from("::1"));

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        //stuff
    }
}
let m = Message::Write(String::from("hello"));
m.call();

//*option enum for when something might be null
enum Option<T> {
    Some(T),
    None,
}
