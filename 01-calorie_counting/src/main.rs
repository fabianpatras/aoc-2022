use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Provide only the input file!");
        return ();
    }

    let file_name: &str = &args[1];

    let file_content = fs::read_to_string(file_name)
        .expect("Could not read input file");

    let lines: Vec<&str> = file_content.split('\n').collect();

    let mut current_calories: u64 = 0;
    let mut spiridush_calories: Vec<u64> = Vec::new();

    for line in lines.iter() {
        if line.len() == 0 {
            spiridush_calories.push(current_calories);
            current_calories = 0;
            continue;
        }
        
        current_calories += line.parse::<u64>().unwrap();
    }

    spiridush_calories.sort();
    spiridush_calories.reverse();
    
    let sum = spiridush_calories
        .iter()
        .take(3)
        .fold(0, |a, b| a + b);

    println!("Calories of top 3 [{}]", sum);
}

