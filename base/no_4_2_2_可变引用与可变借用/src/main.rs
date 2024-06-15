fn main() {
    let mut s = String::from("hello");

    let _s1 = &mut s;
    // let _s2 = &mut s; // 这里会报错，因为 s1 已经借用了 s，所以 s2 不能再借用 s
    // let _s2 = &s; // 这里也会报错，因为可变引用与不可变引用不能同时存在
    // println!("{}, {}", _s1, _s2);

    change(&mut s); // 这里传入的是可变引用，使用 &mut 符号
    println!("{}", s);
}

// s 是一个可变引用，它借用了 String 类型的所有权
fn change(s: &mut String) {
    s.push_str(", world");
}
