pub fn part1(input: String) {
    let mut max_calories: i32 = 0;
    let mut current_elf_calories: i32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            if current_elf_calories > max_calories {
                max_calories = current_elf_calories;
            }
            current_elf_calories = 0;
        } else {
            current_elf_calories += line.parse::<i32>().unwrap();
        }
    }

    println!("{max_calories}");
}

pub fn part2(input: String) {
    let (mut first, mut second, mut third) = (0, 0, 0);
    let mut current_elf_calories: i32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            (first, second, third) = calculate_elf_calories(current_elf_calories, first, second, third);
            current_elf_calories = 0;
        } else {
            current_elf_calories += line.parse::<i32>().unwrap();
        }
    }

    (first, second, third) = calculate_elf_calories(current_elf_calories, first, second, third);

    println!("{}", first + second + third);
}

fn calculate_elf_calories(current: i32, mut first: i32, mut second: i32, mut third: i32) -> (i32, i32, i32) {
    if current > first {
        third = second;
        second = first;
        first = current;
    } else if current > second {
        third = second;
        second = current;
    } else if current > third {
        third = current;
    }

    (first, second, third)
}