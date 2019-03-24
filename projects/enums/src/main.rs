enum IpAddrKind {
    V4,
    V6,
}

struct IpAddrs {
    _kind: IpAddrKind,
    _address: String,
}

enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}


impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

fn main() {
    let _home = IpAddrs {
        _kind: IpAddrKind::V4,
        _address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddrs {
        _kind: IpAddrKind::V6,
        _address: String::from("::1"),
    };

    let _home = IpAddr::V4(String::from("127.0.0.1"));

    let _loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let _some_number = Some(5);
    let _some_string = Some("a string");

    let _absent_number: Option<i32> = None;
 
}
