fn main() {
    let string1 = String::from("长字符1111");
    let string2 = "短字符222";
    let result = longest(string1.as_str(), string2);
    print!("==> {}", result);
}

fn longest<'b>(x: &'b str, y: &'b str) -> &'b str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}