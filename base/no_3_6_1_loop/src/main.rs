fn main() {
    let mut i = 0;
    let result = loop {
        i += 1;
        println!("Hello, world! {} times", i);
        if i >= 5 {
            // 可以带返回值
            break i;
        }
    };
    println!("result: {}", result);
}
