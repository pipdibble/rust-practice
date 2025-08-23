fn consume(s: &String) {
    println!("Consumed: {}", s);
}

fn main() {
    let s = String::from("Rust");
    let r = &s;
    consume(r);
    println!("Still have reference: {}", r);
}
