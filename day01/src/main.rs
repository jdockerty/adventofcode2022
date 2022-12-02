use aoc_utils::read_input;
use std::str::Lines;


fn find_max(lines: Lines) -> u64 {

    let mut highest_calories: u64 = 0;
    let mut current_elf: u64 = 0;

    for line in lines {
        // Do not parse blank lines and reset counter.
        if line == "" {
            if current_elf > highest_calories {
                highest_calories = current_elf;
            }

            current_elf = 0;
            continue;
        }

        current_elf += line.parse::<u64>().expect("should be able to parse line");
    };

    highest_calories
}

fn find_top_three(lines: Lines) -> u64{
    
    let mut calories = Vec::new();
    let mut sum: u64 = 0;

    for line in lines {
        if line == "" {

            calories.push(sum);
            sum = 0;
            continue;
        }

       sum += line.parse::<u64>().expect("should be able to parse line"); 

    }

    // Push final element
    calories.push(sum);

    calories.sort();
    
    // This is disgusting, but it does work :)
    calories[calories.len() - 1] + calories[calories.len() - 2] + calories[calories.len() - 3]
}


fn main() {
    let input = read_input("day01/input/calories.txt".to_string());
    let lines = input.lines();

    println!("Highest: {}", find_max(lines.clone()));

    println!("Top three total: {:?}", find_top_three(lines.clone()))
}
