use std::{fs::File, io::ErrorKind};

fn main() {
    let file = File::open("hello2.txt");
    match file {
        Ok(f) => {
            println!("打开文件成功: {:?}", f);
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello2.txt") {
                Ok(f) => {
                    println!("创建文件成功: {:?}", f);
                }
                Err(e) => {
                    println!("创建文件失败: {:?}", e);
                }
            },
            _ => {
                println!("打开文件失败: {:?}", e);
            }
        },
    }
}
