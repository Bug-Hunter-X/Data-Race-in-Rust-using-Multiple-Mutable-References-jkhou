fn main() {
    let mut x = 5;
    // Solution 1: Cloning
    let y = x;
    let mut z = x;

    let mut y1 = y;
    let mut z1 = z;

    y1 = 6;
    z1 = 7;

    println!("x = {}, y = {}, z = {}", x, y1, z1);

    // Solution 2: Using RefCell
    use std::cell::RefCell;
    let x = RefCell::new(5);
    let mut y = x.borrow_mut();
    *y = 6;
    let mut z = x.borrow_mut();
    *z = 7;
    println!("x = {}", x.borrow());
} 