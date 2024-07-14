fn main() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // x 的值没有被移动，所以可以继续使用，因为 i32 是 Copy 类型

    // 使用Box<T>来存储数据
    let x = Box::new(5);
    let y = x;
    // println!("x = {}", x); // 错误[E0382]: `x` 的所有权已经移动到 `y` 中
    println!("y = {}", y);

    // 使用Box<T>来创建递归类型
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("{:?}", list);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // Box<T> 是一个智能指针，允许在堆上存储数据，而不是栈上，这样就可以在编译时确定数据的大小
    Nil,
}
