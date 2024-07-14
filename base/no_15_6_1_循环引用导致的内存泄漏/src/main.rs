use std::{cell::RefCell, rc::Rc};
use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a 初始 rc 计数 = {}", Rc::strong_count(&a));
    println!("a 下一项 = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("b 创建后 a 的 rc 计数 = {}", Rc::strong_count(&a));
    println!("b 初始 rc 计数 = {}", Rc::strong_count(&b));
    println!("b 下一项 = {:?}", b.tail());

    if let Some(link) = a.tail() {
        // 这里我们将 a 的尾部指向 b，形成一个循环
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a 被改变后 b 的 rc 计数 = {}", Rc::strong_count(&b));
    println!("a 被改变后 a 的 rc 计数 = {}", Rc::strong_count(&a));

    // 将下一行的注释去掉，就可以看到我们形成了一个循环引用，这会导致内存泄漏
    println!("a 下一项 = {:?}", a.tail());
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    /// 用于获取 Cons 中的下一项
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}