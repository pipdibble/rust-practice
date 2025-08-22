fn swap(a: &mut i32, b: &mut i32) {
    let temp: i32 = *a;
    *a = *b;
    *b = temp;
}

fn main() {
    let mut x: i32 = 3;
    let mut y: i32 = 5;
    swap(&mut x, &mut y);
    println!("{x} {y}");
}
