use std::io; 
use rand::Rng;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let numbers: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid number"))
            .collect();

        if numbers.len() < 2 {
            continue; 
        }
        let first_number = numbers[0];
        let end_number = numbers[1];
        if first_number == 0 && end_number == 0 {
            break; 
        }
    
        let mut rng = rand::thread_rng();
        let random_number: i32 = rng.gen_range(first_number..=end_number);
        println!("{}", random_number); 
    }
}