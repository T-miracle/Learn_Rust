fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    if number == 3 || number == 4 {
        println!("number is 3 or 4");
    } else {
        println!("number is not 3 or 4");
    }
}
