fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    println!("r1: {}", r1);
    let r2 = &mut s;
    println!("r2: {}", r2);
}
