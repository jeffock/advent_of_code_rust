const DATA: &str = include_str!("./input1.txt");

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
                    if key_res.contains(&value) && value != ""{
                        count += 1; 
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
