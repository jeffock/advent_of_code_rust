const DATA: &str = include_str!("./input1.txt");

fn main() {
    let ans = DATA.lines().map(|line| {
       let digit10 = first_digit(line.chars());
       let digit1 = first_digit(line.chars().rev());
       digit10*10 + digit1
    }).sum::<i32>();
    println!("{ans}"); 
}

fn first_digit(iterator: impl Iterator<Item=char>) -> i32 {
    iterator.map(|c| c.to_digit(10)).flatten().next().unwrap() as i32
}
