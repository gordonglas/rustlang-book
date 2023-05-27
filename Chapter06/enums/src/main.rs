enum IpAddrKind {
    V4,
    V6,
}

// or with data type String...

// These become ctor functions which take a String param,
// but each (V4 or V6) can have different param types and multiple params,
// and even associate each type with a name, like in a struct.
// We can even define methods on enums via an impl block, like structs.
// See: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
enum IpAddr {
    V4(String),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {
    
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    route(four);
    route(six);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // The "match" syntax to run code based on the type.
    // "ref" is used so we can also use it to demonstrate "if let" below.
    match home {
        IpAddr::V4(ref ipv4) => println!("home: {}", ipv4),
        // Handle all other types by doing nothing.
        _ => (),
    }

    match loopback {
        IpAddr::V6(ref ipv6) => println!("loopback: {}", ipv6),
        // Handle all other types by doing nothing.
        _ => (),
    }

    // The "if let" syntax to run code based on the type.
    if let IpAddr::V4(ipv4) = home {
        println!("home: {}", ipv4);
    }

    if let IpAddr::V6(ipv6) = loopback {
        println!("loopback: {}", ipv6)
    }
}
