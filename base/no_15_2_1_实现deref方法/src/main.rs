use std::ops::Deref;

fn main() {
    let x = MyBox::new(5);
    assert_eq!(*x, 5); // 如果不实现 Deref trait，这里会报错
}

struct MyBox<T> {
    value: T,
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox { value: x }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; // 定义关联类型，用于指定 Deref trait 将会返回的类型

    fn deref(&self) -> &T {
        &self.value
    }
}
