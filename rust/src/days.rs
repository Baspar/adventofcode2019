mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub fn part1 (day: &str, input: &str) -> String {
    return match day {
        "day1" => day1::part1(input),
        "day2" => day2::part1(input),
        "day3" => day3::part1(input),
        "day4" => day4::part1(input),
        "day5" => day5::part1(input),
        "day6" => day6::part1(input),
        "day7" => day7::part1(input),
        "day8" => day8::part1(input),
        "day9" => day9::part1(input),
        "day10" => day10::part1(input),
        "day11" => day11::part1(input),
        "day12" => day12::part1(input),
        "day13" => day13::part1(input),
        "day14" => day14::part1(input),
        "day15" => day15::part1(input),
        "day16" => day16::part1(input),
        "day17" => day17::part1(input),
        "day18" => day18::part1(input),
        "day19" => day19::part1(input),
        "day20" => day20::part1(input),
        "day21" => day21::part1(input),
        "day22" => day22::part1(input),
        "day23" => day23::part1(input),
        "day24" => day24::part1(input),
        "day25" => day25::part1(input),
        _ => String::from("Invalid day")
    }
}
pub fn part2 (day: &str, input: &str) -> String {
    return match day {
        "day1" => day1::part2(input),
        "day2" => day2::part2(input),
        "day3" => day3::part2(input),
        "day4" => day4::part2(input),
        "day5" => day5::part2(input),
        "day6" => day6::part2(input),
        "day7" => day7::part2(input),
        "day8" => day8::part2(input),
        "day9" => day9::part2(input),
        "day10" => day10::part2(input),
        "day11" => day11::part2(input),
        "day12" => day12::part2(input),
        "day13" => day13::part2(input),
        "day14" => day14::part2(input),
        "day15" => day15::part2(input),
        "day16" => day16::part2(input),
        "day17" => day17::part2(input),
        "day18" => day18::part2(input),
        "day19" => day19::part2(input),
        "day20" => day20::part2(input),
        "day21" => day21::part2(input),
        "day22" => day22::part2(input),
        "day23" => day23::part2(input),
        "day24" => day24::part2(input),
        "day25" => day25::part2(input),
        _ => String::from("Invalid day")
    }
}
