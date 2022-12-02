use crate::common::common::read_lines;

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();

    let mut max_calorie = 0;
    let mut calorie = 0;

    let mut elves: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(input_path) {
        for line in lines.flatten() {
            if line.trim() == "" {
                println!("{}, {}", calorie, max_calorie);
                if calorie > max_calorie {
                    max_calorie = calorie;
                    elves.push(calorie);
                }
                calorie = 0;
            } else {
                let n: i32 = line.trim().parse::<i32>().unwrap();

                calorie += n;
            }
        }
    }

    elves.push(calorie);

    println!("{}", max_calorie);

    // solution = max_calorie.to_string();

    elves.sort();

    // println!("{}", elves);
    for elf in elves.clone() {
        println!("LIST: {}", elf);
    }

    solution = (elves[elves.len() - 1] + elves[elves.len() - 2] + elves[elves.len() - 3]).to_string();

    return solution;
}
