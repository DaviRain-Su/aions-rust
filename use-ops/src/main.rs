use ops::addition_ops::add;
use ops::multiplication_ops::div;

fn main() {
    let sum = add(1, 2);
    let div = div(1, 2);
    println!("1 + 2 = {sum}");
    println!("1 / 2 = {div}");
}
