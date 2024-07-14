fn main() {
    let x = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("被创建的值 -- {}", x.data);
    let y = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("被创建的值 -- {}", y.data);
    drop(x); // 显式调用 drop 函数。可以尝试注释掉这行代码，看看 x 的值被删除的时机
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("被删除的值 ----- {}", self.data);
    }
}