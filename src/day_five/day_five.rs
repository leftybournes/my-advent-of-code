use my_advent_of_code::inputs;

const MAX_BOUND: f64 = 127.0;
const MIN_BOUND: f64 = 0.0;
const MAX_ROW: f64 = 7.0;

fn parse_ids(input: &Vec<String>) -> Vec<u32> {
    let mut ids: Vec<u32> = vec![];

    for row in input.iter() {
        let mut current_row = 0;
        let mut current_column = 0;

        let mut upper_bound: f64 = MAX_BOUND;
        let mut lower_bound: f64 = MIN_BOUND;

        let mut upper_row_bound: f64 = MAX_ROW;
        let mut lower_row_bound: f64 = MIN_BOUND;

        for ch in row.chars() {
            match ch {
                'F' => {
                    upper_bound -= ((upper_bound - lower_bound) / 2.0).ceil();
                    current_row = upper_bound as u32;
                }
                'B' => {
                    lower_bound += ((upper_bound - lower_bound) / 2.0).ceil();
                    current_row = lower_bound as u32;
                }
                'L' => {
                    upper_row_bound -= ((upper_row_bound - lower_row_bound) / 2.0).ceil();
                    current_column = upper_row_bound as u32;
                }
                'R' => {
                    lower_row_bound += ((upper_row_bound - lower_row_bound) / 2.0).ceil();
                    current_column = lower_row_bound as u32;
                }
                _ => {
                    println!("Cannot parse character {}", ch);
                }
            }
        }

        ids.push(current_row * 8 + current_column);
    }

    ids
}

fn part_one(ids: &Vec<u32>) -> Option<u32> {
    ids.to_owned().into_iter().max()
}

fn part_two(ids: &Vec<u32>) -> u32 {
    // seat isn't at the front or back which means ids 0 and len - 1 can be excluded from check
    let mut seat_id: u32 = 0;
    let mut sorted_ids = ids.to_owned();
    sorted_ids.sort();

    let inclusive_ids = sorted_ids[1..(ids.len() - 2)].to_owned();
    let mut boolarr: Vec<bool> = vec![false; inclusive_ids.len() + 1];

    let min_id = inclusive_ids[0];
    for id in inclusive_ids.iter() {
        boolarr[(id - min_id) as usize] = true;
    }

    for (index, value) in boolarr.iter().enumerate() {
        if !value {
            seat_id = index as u32 + min_id;
            break;
        }
    }

    if inclusive_ids.contains(&(seat_id + 1)) && inclusive_ids.contains(&(seat_id - 1)) {
        seat_id
    } else {
        0
    }
}

fn main() {
    let path = String::from("inputs/day_five/input.txt");
    let input = inputs::parse_input(path)
        .expect("Error parsing input")
        .lines()
        .map(|row| row.parse::<String>().unwrap())
        .collect();

    let ids = parse_ids(&input);
    let max_id = part_one(&ids);
    match max_id {
        Some(id) => println!("Max ID: {}", id),
        None => println!("The max id could not be found"),
    };

    let seat_id = part_two(&ids);
    println!("Seat ID: {}", seat_id);
}
