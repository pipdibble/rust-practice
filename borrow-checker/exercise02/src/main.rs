fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
   let x_length = x.len();
   let y_length = y.len();
   if x_length > y_length {
       return x;
   } else {
       return y;
   }
}
fn main() {
    let str1 = "apple";
    let str2 = "banana";
    let long_result = longest(str1, str2);
    println!("{long_result}");
}
