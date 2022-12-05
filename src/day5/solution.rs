use std::cmp::min;
use crate::common::common::read_lines;

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();

    let mut stacks: Vec<Vec<char>> = vec![];

    let mut first = true;
    let mut instruction_part = true;

    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                println!("{}", line);

                if instruction_part {
                    let (p, _) = line.split_at(2);
                    if p.eq(" 1") {
                        instruction_part = false;
                        print_stacks(&stacks.clone());

                        continue;
                    }

                    let mut row = line.clone();
                    let mut i = 0;

                    loop {
                        if row.eq("") {
                            break;
                        }

                        if first {
                            stacks.push(vec![] as Vec<char>);
                        }

                        let (section, next_row) = row.split_at(min(4, row.len()));

                        let c = section.chars().collect::<Vec<char>>()[1];

                        if c != ' ' {
                            if stacks[i].len() > 0 {
                                let top = stacks[i][0];
                                stacks[i].splice(0..1, vec![c, top]);
                            } else {
                                stacks[i].push(c);
                            }
                        }

                        row = next_row.to_string();
                        i += 1;
                    }
                } else {
                    let v = line.split(' ').collect::<Vec<&str>>();

                    if v.len() >= 6 {
                        let m: usize = v[1].parse().unwrap();
                        let f: usize = v[3].parse::<usize>().unwrap() - 1;
                        let t: usize = v[5].parse::<usize>().unwrap() - 1;
                        println!("MOVE {} FROM {} TO {}", m, f, t);

                        let mut tmp_v = vec![];

                        for _ in 0..m {
                            let from_stack = &mut stacks[f];

                            let c: char = from_stack.pop().unwrap();

                            tmp_v.push(c);
                        }

                        for _ in 0..m {
                            let to_stack = &mut stacks[t];

                            to_stack.push(tmp_v.pop().unwrap());
                        }
                    }
                }
            }

            first = false;
        }
    }


    for mut e in stacks {
        solution.push(e.pop().unwrap());
    }

    // solution = ...;

    return solution;
}

fn print_stacks(stacks: &Vec<Vec<char>>) {
    let mut i = 0;
    for stack in stacks {
        print!("{}: ", i);
        for c in stack {
            print!("{}", c);
        }
        println!();

        i += 1;
    }
}
