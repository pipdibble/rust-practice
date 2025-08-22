fn process(slice: &mut [i32]) {
    let length = slice.len();
    let (slice_1, slice_2) = slice.split_at_mut(length/2);
    slice_1[0] = slice_1[0] * 2;
    println!("slice_1: {:?}", slice_1);
    println!("slice_2: {:?}", slice_2);
    

}

fn main() {
    let mut x = [1, 2, 3, 4, 5, 6];
    process(&mut x);
    println!("{:?}", x);
}
