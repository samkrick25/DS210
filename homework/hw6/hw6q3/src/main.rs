use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    let number: u32 = input.parse().expect("Not a good number!");
    let mut sum: u128 = 0;
    for i in 1..number+1 {
        let i:u128 = i as u128;
        sum += i.pow(2);
    }
    println!("The sum of squares from 1 to your input is {}", sum)
}
