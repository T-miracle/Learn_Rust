use std::io;

fn main() {
    original_writing_method();

    let mut vec = use_macro();

    // 添加元素
    vec.push(4);
    println!("添加了一个新的元素：{:?}", vec);

    // 删除第2个元素
    vec.remove(1);
    println!("移除了第2个元素：{:?}", vec);

    // 获取第3个元素
    let third = &vec[2];
    println!("获取第3个元素：{}", third);

    println!("\n写一个索引值来获取元素：");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("读取失败");
    let index: usize = index.trim().parse().expect("请输入一个数字");
    // 使用模式匹配
    match vec.get(index) {
        Some(i) => println!("使用模式匹配获取元素：{}", i),
        None => println!("没有找到该元素"),
    }
}

/**
 * 使用原始方式创建一个数组迭代器
 */
fn original_writing_method() {
    // let vec: Vec<i32> = Vec::new();
    let vec: Vec<i32> = Vec::from([1, 2, 3]);
    println!("\n使用原始方式创建了一个数组迭代器：{:?} \n", vec);
}

/**
 * 使用宏创建一个数组迭代器
 */
fn use_macro() -> Vec<i32> {
    let vec = vec![1, 2, 3];
    println!("使用宏创建了一个数组迭代器：{:?}", vec);
    vec
}
