fn main() {
    let x = 10;
    let y = &x; // 这里 y 是 x 的引用，y 借用了 x (获取变量的引用，称之为借用)
    println!("x: {}, y: {}, *y: {}", x, y, *y);

    assert_eq!(10, x);
    assert_eq!(10, *y);

    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 这里传入的是引用，使用 & 符号
    println!("The length of '{}' is {}.", s1, len);
}

// 这里 &String 表示 s 是一个引用，而不是所有权
// 使用 s 被称为借用
fn calculate_length(s: &String) -> usize {
    s.len()
}
