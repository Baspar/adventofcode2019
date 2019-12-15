use std::env;
use std::process;
use std::fs;
use regex::Regex;
use std::time::Instant;

mod days;
mod intcode;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check given day
    if args.len() <= 1 {
        println!("Please provide a day");
        process::exit(1)
    }

    // Check if given day is valid
    let day: &String = &args[1];
    let day_regex = Regex::new(r"^day([1-9]|1[0-9]|2[0-5])$").unwrap();
    if !day_regex.is_match(day) {
        println!("Please provide a valid day");
        process::exit(1)
    }

    // Load file
    let filename: String = format!("./inputs/{}.txt", day);
    println!("{}", filename);
    let file_input = fs::read_to_string(filename).expect("cannot read file");

    // Retrieve input
    let input = if args.len() > 2 {
        // Use Args as input for day
        &args[2]
    } else {
        &file_input
    };
    println!("{}\n", input);


    // Part1
    print!("Part1: ");
    let start1 = Instant::now();
    let result_part1 = days::part1(day, &input);
    let duration1 = start1.elapsed();
    println!("({:?})", duration1);
    println!("{}\n", result_part1);


    // Part2
    print!("Part2: ");
    let start2 = Instant::now();
    let result_part2 = days::part2(day, &input);
    let duration2 = start2.elapsed();
    println!("({:?})", duration2);
    println!("{}\n", result_part2);

    println!("Time elapsed: {:?}", duration1 + duration2);
}
