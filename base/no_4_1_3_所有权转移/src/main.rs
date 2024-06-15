fn main() {
    let s1 = give_ownership(); // s1 获得了 give_ownership 的返回值
    println!("{}", s1);

    let s2 = String::from("hello");
    println!("s2: {}", s2);

    let s3 = take_and_give_back(s2); // s2 的所有权被移动到 take_and_give_back 中，然后又被移动给了 s3，这导致 s2 失去了所有权

    // println!("s2: {}", s2); // s2 已经被移动，这里会报错
    println!("s3: {}", s3);
}

fn give_ownership() -> String {
    let s = String::from("hello");
    s
} // s 在此处跳出作用域，但是 s 的所有权移动给了调用者 s1

fn take_and_give_back(s: String) -> String {
    s
} // s 在此处跳出作用域，但是 s 的所有权移动给了调用者 s3
