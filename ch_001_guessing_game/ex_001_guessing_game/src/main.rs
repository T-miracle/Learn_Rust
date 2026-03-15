use std::cmp::Ordering;
use std::io::stdin;
use rand::RngExt;

/// 猜数字游戏
fn main() {
    println!("猜数字游戏~~");
    // 使用rand库生成一个随机数字
    let secret_number = rand::rng().random_range(1..100);
    // 循环输入，直到相等为止
    loop {
        println!("\n请输入一个数字：");
        // 定义一个变量
        let mut guess = String::new();
        // 调用输入
        stdin().read_line(&mut guess).expect("没有检测到输入！");
        // 变量遮蔽
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("输入的不是数字，请重新输入！");
                continue;
            }
        };
        println!("你猜的是: {}", guess);
        // 进行判断
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("数字小了！"),
            Ordering::Greater => println!("数字大了！"),
            Ordering::Equal => {
                println!("数字相等！你猜对了！！！");
                break;
            }
        }
    }
}
