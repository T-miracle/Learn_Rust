use std::{fs::File, io::ErrorKind};

/// 使用 `unwrap_or_else` 代替 `match` 处理 `Result` 类型
fn main() {
    let file = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("创建文件失败: {:?}", error);
            })
        } else {
            panic!("打开文件失败: {:?}", error);
        }
    });
    println!("文件: {:?}", file);
}
