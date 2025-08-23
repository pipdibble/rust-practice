fn process(slice: &mut [i32]) {
    let (slice1, slice2) = slice.split_at_mut(slice.len()/2);
    for elem in slice1 {
        *elem *= 2;
    }
    for elem in slice2 {
        *elem += 10;
    }
}

fn main() {
    let mut x = [1, 2, 3, 4, 5, 6];
    process(&mut x);
    println!("{:?}", x);
}
