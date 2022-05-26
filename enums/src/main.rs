enum IpAddr {
    // V4(String),
    V4(u8,u8,u8,u8),
    V6(String),
}
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn main() {

    // detail of how enums work: 
    // the name of each enum variant that we define also becomes a function that constructs an instance of the enum. 
    // That is, IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type. 
    // We automatically get this constructor function defined as a result of defining the enum.
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
}