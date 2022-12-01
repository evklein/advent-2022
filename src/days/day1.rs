use std::thread::current;

pub fn part1(input: String) {
    println!("Running Day 1 - Part 1");

    let mut max_calories: i32 = 0;
    let mut current_elf_calories: i32 = 0;

    for line in input.lines() {
        if (line.is_empty()) {
            if (current_elf_calories > max_calories) {
                max_calories = current_elf_calories;
            }
            current_elf_calories = 0;
        } else {
            current_elf_calories += line.parse::<i32>().unwrap();
        }
    }

    println!("{max_calories}");
}