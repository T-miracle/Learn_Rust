use std::cell::RefCell;

fn main() {
    let msg_queue = MsgQueue {
        msg_cache: RefCell::new(Vec::new()),
    };
    msg_queue.send(String::from("hello"));
    println!("{:?}", msg_queue.msg_cache);
    println!("{:?}", msg_queue.msg_cache.borrow());
}

// 定义在外部库中的特征
pub trait Messenger {
    fn send(&self, msg: String);
}

// --------------------------
// 我们的代码中的数据结构和实现
struct MsgQueue {
    // msg_cache: Vec<String>,
    msg_cache: RefCell<Vec<String>>,
}

impl Messenger for MsgQueue {
    fn send(&self, msg: String) {
        // self.msg_cache.push(msg) // 如果 `msg_cache` 是一个 `Vec<String>` 类型的字段, 这里会报错，因为 self 是不可变引用
        self.msg_cache.borrow_mut().push(msg); // borrow_mut() 返回一个 RefMut, 用于修改 RefCell 中的值
    }
}
