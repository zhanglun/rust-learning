fn main() {
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }
    // println!("Hello, world!");

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1")
    // };

    // 可以使用一种更简洁的方式来表达相同的概念，
    // 仅仅使用枚举并将数据直接放进每一个枚举成员而不是将枚举作为结构体的一部分。
    // IpAddr 枚举的新定义表明了 V4 和 V6 成员都关联了 String 值
    

    // enum IpAddrKind {
    //     V4(String),
    //     V6(String)
    // }

    // let home = IpAddrKind::V4(String::from("127.0.0.1"));
    // let loopback = IpAddrKind::V6(String::from("::1"));

    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    // 在rust中甚至可以将结构体传入枚举中
    struct Ipv4Addr {
        // --snip--
    }
    
    struct Ipv6Addr {
        // --snip--
    }
    
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

    // 枚举还可以使用`impl` 在枚举上定义方法，就像结构体一样
    impl Message {
        fn call(&self) {
            println!("我调用了枚举上定义的call方法！");
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();


    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // Option<T> 定义在标准库中 https://doc.rust-lang.org/std/option/enum.Option.html
    let some_number = Some(5);
    let some_string = Some("a string");
    
    let absent_number: Option<i32> = None;
}