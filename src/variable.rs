pub fn string() {
    let missiles = 8;
    let ready = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);
}

pub fn mutable() {
    let _y = 6; // imutable default
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

pub fn constants() {
    const MAX: u32 = 5;
    println!("The value of MAX is: {}", MAX);
}

pub fn shadowing() {
    // Khác với mutable ở chỗ có thể sử dụng lại tên và có thể thay đổi kiểu của biến
    let x = 5;

    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);
}
