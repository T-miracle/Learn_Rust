/**
 * 切片（slice）是一种引用类型，它允许你引用集合的一部分序列，而无需拥有其所有权
 */
fn main() {
    /* 数组切片 */
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3]; // 引用 arr 的一部分，即 [2, 3]
    println!("{:?}", slice);

    /* 字符串切片 */
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);

    let s = "hello world";
    let word = first_word(&s);
    println!("{}", word);

    /* 可变的切片 */
    let mut arr = [1, 2, 3, 4, 5];
    let slice = &mut arr[1..4]; // 可变切片
    for elem in slice.iter_mut() {
        *elem += 10; // 修改切片中的每个元素
    }
    println!("{:?}", slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
