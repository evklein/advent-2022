pub fn part1(input: String) {
    let mut priority_sum: i32 = 0;
    let mut duplicates: Vec<char> = vec![];

    input.lines()
        .map(|line| line.split_at(line.len() / 2))
        .for_each(|(first, second)| {
            for c in first.chars() {
                if second.contains(c) {
                    duplicates.push(c);
                    break;
                }
            }
        });

    priority_sum = duplicates.iter()
                             .map(|c| {
                                if c.is_lowercase() { (c.to_owned() as i32) - 96 }
                                else { (c.to_owned() as i32) - 38 }
                             }).sum();
    println!("{:?}", priority_sum);
}

pub fn part2(input: String) {
    
}
