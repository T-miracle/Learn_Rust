use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf 强引用计数 = {}, 弱引用计数 = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    println!("leaf 的父节点 = {:?}", leaf.parent.borrow().upgrade());

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "\nleaf 强引用计数 = {}, 弱引用计数 = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
        println!("leaf 的父节点 = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "branch 强引用计数 = {}, 弱引用计数 = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
    }

    println!("\nleaf 的强引用计数 = {}, 弱引用计数 = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("leaf 的父节点 = {:?}", leaf.parent.borrow().upgrade());
}
