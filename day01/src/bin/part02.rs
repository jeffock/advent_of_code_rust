fn main() {
    let input = include_str!("./example2.txt");
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines = input.lines();
    let mut sum = 0;

    for line in lines {
        let len = line.len();
        let mut strip = line;
        for i in 0..len {
            let str_to_replace3 = &strip[i..i+3];
            match str_to_replace3 {
                _ if str_to_replace3 == "one" => strip.replace_range(i..(i+3),"1"),
                _ if str_to_replace3 == "two" => strip.replace_range(i..(i+3),"2"),
                _ if str_to_replace3 == "six" => strip.replace_range(i..(i+3),"6"),
            }
            let str_to_replace4 = &strip[i..i+4];
            if assert_eq!(str_to_replace4,"four")||assert_eq!(str_to_replace4,"five")||assert_eq!(str_to_replace4,"nine") {
                if str_to_replace4=="four" {
                    strip.replace_range(i..i+4,"4");
                } else if str_to_replace4=="five" {
                    strip.replace_range(i..i+4,"5");
                } else if str_to_replace4=="nine" {
                    strip.replace_range(i..i+4,"9");
                }
            }
            let str_to_replace5 = &strip[i..i+5];
            if assert_eq!(str_to_replace5,"three")||assert_eq!(str_to_replace5,"seven")||assert_eq!(str_to_replace5,"eight") {
                if str_to_replace5=="three" {
                    strip.replace_range(i..i+5,"3");
                } else if str_to_replace5=="seven" {
                    strip.replace_range(i..i+5,"7");
                } else if str_to_replace5=="eight" {
                    strip.replace_range(i..i+5,"8");
                }
            }
        }

        strip.replace(&['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'][..], "");

        let first = strip.chars().next().unwrap();
        let last = strip.chars().last().unwrap();
       
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
