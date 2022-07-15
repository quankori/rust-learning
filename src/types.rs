pub fn data() {
    // Integer
    let guess: u32 = 10; // i8 ... i128 ... isize || u8 ... u128 ... usize

    // Float
    let x = 2.0; //f64
    let y: f32 = 3.0; //f32

    // Boolean
    let t = true;
    let f: bool = false;

    // Char
    let c = 'z';
    println!("{}", guess);
    println!("{}", x);
    println!("{}", y);
    println!("{}", t);
    println!("{}", f);
    println!("{}", c);
}

pub fn tuple() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    println!("{}", five_hundred);
    println!("{}", six_point_four);
    println!("{}", one);
}

pub fn array() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("{}", first);
    println!("{}", second);
}
pub fn enum_struct() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
