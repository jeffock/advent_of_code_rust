fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines = input.lines();
    let mut sum = 0;

    for line in lines {
        let stripped = line.replace(&['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'][..], "");

        let first = stripped.chars().next().unwrap();
        let last = stripped.chars().last().unwrap();
       
        let mut value = String::from("");
        value.push(first);
        value.push(last);

        let value_i: i32 = value.parse().unwrap();

        sum += value_i;
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    // get part1 from parent module  
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("");
        assert_eq!(result, "4".to_string());
    }
}
