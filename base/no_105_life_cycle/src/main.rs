fn main() {
    let string1 = String::from("this is a long string");
    let string2 = "this is a short string";
    let result = longest(string1.as_str(), string2);
    print!("==> {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}