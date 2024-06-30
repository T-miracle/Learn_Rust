use std::collections::HashMap;

fn main() {
    // 基本用法
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    // 使用 collect 方法合并两个 vector
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);

    // 获取值
    let score = scores.get(&String::from("Blue"));

    match score {
        Some(value) => println!("The value is: {}", value),
        None => println!("The value is not found"),
    }
}
