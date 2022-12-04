use crate::common::common::read_lines;

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();

    let mut score = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            let line = line_result.unwrap();

            let ranges: Vec<&str> = line.split(',').collect();

            let range1: Vec<i32> = ranges[0].split('-').map(|s| s.parse::<i32>().unwrap()).collect();
            let range2: Vec<i32> = ranges[1].split('-').map(|s| s.parse::<i32>().unwrap()).collect();

            // if (range1[0] <= range2[0] && range1[1] >= range2[1]) || (range2[0] <= range1[0] && range2[1] >= range1[1]) {
            if !((range2[0] > range1[0] && range2[0] > range1[1]) || (range1[0] > range2[0] && range1[0] > range2[1])) {
                score += 1;
            }
        }
    }

    solution = score.to_string();

    return solution;
}
