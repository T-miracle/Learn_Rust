use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let msg = read_username_from_file2();

    match msg {
        Ok(s) => println!("文件内容: \n{}", s),
        Err(e) => println!("读取文件失败: \n{:?}", e),
    }
}

/// 返回 `Result` 类型
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;

//     let mut s = String::new();

//     f.read_to_string(&mut s)?;

//     Ok(s)
// }

/// 更加简写的方式
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}