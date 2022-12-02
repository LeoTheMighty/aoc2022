use crate::common::common::read_lines;

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();

    let mut score = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                let parts: Vec<&str> = line.split(' ').collect();

                let opponent = parts[0];
                let me = parts[1];

                if me.eq("X") {
                    if opponent.eq("A") {
                        score += 3;
                    } else if opponent.eq("B") {
                        score += 1;
                    } else if opponent.eq("C") {
                        score += 2;
                    }
                } else if me.eq("Y") {
                    score += 3;
                    if opponent.eq("A") {
                        score += 1;
                    } else if opponent.eq("B") {
                        score += 2;
                    } else if opponent.eq("C") {
                        score += 3;
                    }
                } else if me.eq("Z") {
                   score += 6;
                    if opponent.eq("A") {
                        score += 2;
                    } else if opponent.eq("B") {
                        score += 3;
                    } else if opponent.eq("C") {
                        score += 1;
                    }
                }
            }
        }
    }

    solution = score.to_string();

    return solution;
}
