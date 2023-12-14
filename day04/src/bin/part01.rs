const DATA: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

fn main () {
    let mut sum = 0;

    for line in DATA.lines() {
        let mut count: i32 = -1; 
        let num_only = line.split(':').skip(1).collect::<Vec<&str>>().join("");
        
        let mut no_pipe: Vec<&str> = num_only.split("|").collect();
        
        // refutable binding compiler errer?
        if let Some(key) = no_pipe.get_mut(0) {
            let key_res: Vec<&str> = key.split(' ').collect(); 
            if let Some(values) = no_pipe.get_mut(1) {
                let values_res: Vec<&str> = values.split(' ').collect(); 

                for value in values_res {
                    match key_res.contains(&value) {
                        true => count += 1,
                        false => count += 0,
                    }
                }
                if count >= 0 {
                    sum += i32::pow(2, count as u32);
                }
            }
        }
    }

    // getting 82 not 13?
    println!("{sum}");
}
