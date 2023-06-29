// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let number = -3;
    let answer = square(number);
    println!("The square of {} is {}", number, answer);
}

fn square(num: i32) -> i32 {
    num * num
}
