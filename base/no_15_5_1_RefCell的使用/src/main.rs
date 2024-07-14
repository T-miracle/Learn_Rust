use std::cell::RefCell;

fn main() {
    // err_demo();
    ok_demo();
}

// 一个错误的例子，同时使用了不可变引用和可变引用，会导致运行时错误
fn err_demo() {
    let s = RefCell::new(String::from("hello, world"));
    let s1 = s.borrow();
    let s2 = s.borrow_mut();

    println!("{},{}", s1, s2);
}

// 一个正确的例子，同时使用了两个不可变引用
fn ok_demo() {
    let s = RefCell::new(String::from("hello, world"));
    let s1 = s.borrow();
    let s2 = s.borrow();

    println!("{},{}", s1, s2);
}