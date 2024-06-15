fn main() {
    let s = String::from("hello");
    take_ownership(s);
    // println!("{}", s); // 错误：值被移动到 take_ownership 方法里了，不能再使用
    let x = 5;
    make_copy(x);
    println!("{}", x); // 正确：i32 类型是 Copy 的，它存在于栈上，不会被清理掉，所以 x 仍然有效
}

fn take_ownership(s: String) {
    println!("{}", s);
} // s 被销毁

fn make_copy(i: i32) {
    println!("{}", i);
}
