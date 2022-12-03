use crate::common::common::read_lines;

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();

    let mut score = 0;
    let mut sacks: Vec<String> = vec![];
    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                sacks.push(line);

                if (sacks.len() == 3) {
                    println!("running here");
                    let mut c1: Vec<char> = sacks[0].chars().collect();
                    let mut c2: Vec<char> = sacks[1].chars().collect();
                    let mut c3: Vec<char> = sacks[2].chars().collect();

                    c1.dedup();
                    c2.dedup();
                    c3.dedup();

                    for c in c1 {
                        if c2.contains(&c) && c3.contains(&c) {
                            score += get_score(c);

                            break;
                        }
                    }

                    sacks = vec![];
                }

                // let (compartment1, compartment2) = line.split_at(line.len() / 2);
                //
                // let mut v1: Vec<char> = compartment1.chars().collect();
                // let mut v2: Vec<char> = compartment2.chars().collect();
                //
                // v1.dedup();
                // v2.dedup();
                //
                // for c in v1 {
                //     if v2.contains(&c) {
                //         score += get_score(c);
                //
                //         break;
                //     }
                // }


            }
        }
    }

    solution = score.to_string();

    return solution;
}

fn get_score(c: char) -> u32 {
    println!("{}", c);

    if c.is_lowercase() {
        return (c as u32) - ('a' as u32) + 1;
    }

    return (c as u32) - ('A' as u32) + 27;
}
