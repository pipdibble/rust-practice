
# Rust Borrow Checker Practice Exercises

Below are five focused exercises designed to sharpen your understanding of Rust’s borrow checker. For each exercise, write a **small but complete** Rust program that compiles without errors and demonstrates correct borrowing or ownership. Keep dependencies minimal—only use core language features and `println!`.

---

## Exercise 1: Swap with Mutable References

Write a complete program that:

1. Defines a function  
   ```rust
   fn swap(a: &mut i32, b: &mut i32)
   ```  
   which swaps the values behind two mutable references using only dereferencing and assignments.
2. In `main`, declares two mutable variables `x` and `y`, prints their values before swapping, calls `swap(&mut x, &mut y)`, then prints them after swapping.

This practices basic mutable referencing and dereferencing.

---

## Exercise 2: Longest of Two String Slices

Write a complete program that:

1. Defines a function  
   ```rust
   fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
   ```  
   which returns the longer of two string slices.
2. In `main`, creates two string literals (e.g. `"apple"` and `"banana"`), calls `longest`, and prints the result.

This exercises lifetime annotations and immutable borrowing.

---

## Exercise 3: Scoped Borrowing to Avoid Conflicts

Starting code (won’t compile):
```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    println!("r1: {}", r1);
    let r2 = &mut s;
    println!("r2: {}", r2);
}
```

Rewrite it as a complete, compiling program by adjusting scopes so that the immutable borrow (`r1`) is no longer used when you create the mutable borrow (`r2`). Ensure both prints still occur.

---

## Exercise 4: Splitting a Mutable Slice

Write a complete program that:

1. Defines a function  
   ```rust
   fn process(slice: &mut [i32])
   ```  
   which:
   - Splits `slice` into two non-overlapping mutable sub-slices (first half and second half).
   - Multiplies each element in the first half by 2.
   - Adds 10 to each element in the second half.
2. In `main`, creates a mutable array `[1, 2, 3, 4, 5, 6]`, calls `process(&mut array)`, then prints the modified array.

This practices safe splitting of mutable borrows.

---

## Exercise 5: Borrow vs. Move Resolution

Starting code (won’t compile):
```rust
fn consume(s: String) {
    println!("Consumed: {}", s);
}

fn main() {
    let s = String::from("Rust");
    let r = &s;
    consume(s);
    println!("Still have reference: {}", r);
}
```

Fix it by writing a complete program that compiles. You may choose to:
- Reorder statements so that the borrow doesn’t overlap the move, or  
- Clone the string before moving, or  
- Change `consume` to borrow instead of take ownership.

---

Once you’ve written each program, try intentionally introducing a conflict (e.g., two simultaneous mutable borrows) and observe the compiler errors. That trial-and-error will deepen your grasp of Rust’s borrow checker. Good luck!
