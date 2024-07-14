use std::cell::Cell;

fn main() {
    use_cell_demo();
}

fn use_cell_demo() {
    // 代码片段1 不会报错，Cell是一个线程安全的容器，可以在不可变引用的情况下修改值
    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    x.set(2);
    y.set(3);
    z.set(4);
    println!("最终 x 的值为 {}", x.get());

    // 代码片段2 会报错
    // let mut x = 1;
    // let y = &mut x;
    // let z = &mut x;
    // x = 2;
    // *y = 3;
    // *z = 4;
    // println!("{}", x);
}
