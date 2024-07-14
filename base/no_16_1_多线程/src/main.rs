use std::{thread, time::Duration};

fn main() {
    // demo_1();
    // demo_2();
    demo_3();
}

// 这里我们创建了两个线程，一个在线程中循环，一个在主线程中循环
// 通过运行结果我们可以看到，两个线程是并发执行的
// 但是，主线程执行完毕后，程序就退出了，而在线程中的循环并没有执行完毕
fn demo_1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("在线程中循环的值 ---> {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("在主线程中循环的值 ---> {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// 这里我们通过 handle.join().unwrap() 来等待线程执行完毕
// 这样，主线程就会等待线程执行完毕后再退出
fn demo_2() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("在线程中循环的值 ---> {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("在主线程中循环的值 ---> {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

// 这里我们通过 move 关键字来将 v 移动到线程中
fn demo_3() {
    let v = vec![1, 2, 3];

    // move 关键字表示 v 移动到线程中，这样线程就拥有了 v 的所有权；否则线程无法访问 v，会报错
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}