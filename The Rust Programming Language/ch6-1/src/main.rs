fn main() {
    println!("ch6-1!");

    // Defining an Enum
    enum IpAddrKind {
        V4,
        V6,
    }

    // Enum Values
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind) {}
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    struct IpAddr0 {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr0 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr0 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    enum IpAddr1 {
        V4(String),
        V6(String),
    }

    let home = IpAddr1::V4(String::from("127.0.0.1"));
    let loopback = IpAddr1::V6(String::from("::1"));

    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));

    struct Ipv4Addr {}
    struct Ipv6Addr {}
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // The Option Enum and Its Advantages Over Null Values
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;
}
