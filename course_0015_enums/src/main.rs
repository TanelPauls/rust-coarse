// Enums

// Enum is a tool that is used to represent a type that can take on one of several possible variants.

fn main() {
    enum IpAddrKind{
        V4,
        V6
    }
    let _four: IpAddrKind = IpAddrKind::V4;
    let _six: IpAddrKind = IpAddrKind::V6;

    fn route(_ip_kind: IpAddrKind){}

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    //Using structs

    struct IpAddr{
        kind: IpAddrKind,
        address: String
    };

    let home: IpAddr = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback: IpAddr = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    enum IpAddrKind2 {
        V4(String),
        V6(String)
    };

    let home2: IpAddrKind2 = IpAddrKind2::V4(String::from("127.0.0.1"));
    let loopback2: IpAddrKind2 = IpAddrKind2::V6(String::from("::1"));


    // enchanged enums
    
    enum IpAddrKind3 {
        V4(u8,u8,u8,u8),
        V6(String)
    };

    let home3: IpAddrKind3 = IpAddrKind3::V4(127,0,0,1);
    let loopback3: IpAddrKind3 = IpAddrKind3::V6(String::from("::1"));
}
