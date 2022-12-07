pub fn part1(input: String) {
    // A = Rock, B = Paper, C = Scissors
    // X = Rock, Y = Paper, Z = Scissors
    // 1 = Rock, 2 = Paper, 3 = Scissors
    // Round score: Shape Score + Outcome (0 loss, 3 draw, 6 win)
    let mut total_score: i32 = 0;
    
    input.lines()
         .map(|line| line.split_once(' ').unwrap())
         .for_each(|(o, p)| {
            match o {
                "A" => {
                    match p {
                        "X" => total_score += 1 + 3, // Draw
                        "Y" => total_score += 2 + 6, // Win
                        "Z" => total_score += 3 + 0, // Loss
                        _ => println!("Error"),
                    }
                },
                "B" => {
                    match p {
                        "X" => total_score += 1 + 0, // Loss
                        "Y" => total_score += 2 + 3, // Draw
                        "Z" => total_score += 3 + 6, // Win
                        _ => println!("Error"),
                    }
                },
                "C" => {
                    match p {
                        "X" => total_score += 1 + 6, // Win
                        "Y" => total_score += 2 + 0, // Loss
                        "Z" => total_score += 3 + 3, // Draw
                        _ => println!("Error"),
                    }
                },
                _ => println!("Error"),
            }
         });

    println!("Total score: {total_score}");
}
