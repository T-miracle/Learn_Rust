// 泛型
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let str_list = vec!["abc", "def", "xyz"];
    let result = largest_str(&str_list);
    print!("The largest string is {}", result);
}

// 这里 T: PartialOrd + Copy 表示类型 T 必须同时实现 PartialOrd 和 Copy 两个 trait。
// PartialOrd 允许比较大小，Copy 允许值的复制。
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_str<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > &largest {
            // 注意这里的 &largest
            largest = item;
        }
    }

    largest
}
