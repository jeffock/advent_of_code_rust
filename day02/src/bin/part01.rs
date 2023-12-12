fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> String {
    "todo".to_string()
}

#[cfg(test)]
mod tests {
    // get part1 from parent module  
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("");
        assert_eq!(result, "todo".to_string());
    }
}
