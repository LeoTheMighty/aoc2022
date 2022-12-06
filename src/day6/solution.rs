use crate::common::common::read_lines;

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();

    let mut buffer: Vec<char> = vec![];
    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                let mut i = 1;
                for c in line.chars() {
                    if buffer.len() == 14 {
                        let next_bottom = buffer[1];

                        buffer.splice(0..buffer.len(), pop_front(&buffer, c));

                        let mut d = copy(&buffer);
                        d.sort_unstable();
                        d.dedup();
                        println!("BUFFER: {:?}", buffer);

                        println!("{:?}", d);
                        // print!("COPY: [");
                        // for ch in copy.as_slice().into_iter() {
                        //     print!("{}, ", ch);
                        // }
                        // println!("]");
                        if d.len() == 14 {
                            println!("GOT IT: {}", i);
                            solution = i.to_string();
                            break;
                        }
                    } else {
                        buffer.push(c);
                    }

                    i += 1;
                }
            }
        }
    }

    return solution;
}

fn pop_front(v: &Vec<char>, c: char) -> Vec<char> {
    let mut new_v = vec![];
    let mut first = true;
    for ch in v {
        if !first {
            new_v.push(ch.to_owned());
        }

        first = false;
    }
    new_v.push(c.to_owned());
    return new_v;
}

fn copy(v: &Vec<char>) -> Vec<char> {
   let mut new_v = vec![];
    for c in v {
        new_v.push(c.to_owned());
    }
    return new_v;
}
