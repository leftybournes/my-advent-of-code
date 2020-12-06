use my_advent_of_code::inputs;

fn parse_row(input: &String) -> Vec<String> {
    input.split(' ').map(|c| c.parse().unwrap()).collect()
}

fn parse_bounds(input: &String) -> Vec<u32> {
    input
        .split('-')
        .map(|num_str| num_str.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn parse_test_char(input: &String) -> String {
    input.chars().filter(|&c| c != ':').collect()
}

fn part_one(rows: &Vec<String>) {
    let mut valid_count = 0;

    for row in rows.iter() {
        let columns: Vec<String> = parse_row(&row);

        // Bounds set for the count of the specified character in each password
        // bounds[0] is the lower bound while bounds[1] is the uppoer bound
        let bounds = parse_bounds(&columns[0]);

        // character to test against
        let test_char = parse_test_char(&columns[1]);

        let password = String::from(&columns[2]);

        let mut char_count = 0;

        for c in password.chars() {
            if c.to_string() == test_char {
                char_count += 1
            }
        }

        if char_count >= bounds[0] && char_count <= bounds[1] {
            valid_count += 1;
        }
    }

    println!("The number of valid passwords is {}", valid_count);
}

fn part_two(rows: &Vec<String>) {
    let mut valid_count = 0;

    for row in rows.iter() {
        let columns = parse_row(&row);
        let bounds = parse_bounds(&columns[0]);
        let test_char = parse_test_char(&columns[1]);
        let password: Vec<char> = String::from(&columns[2]).chars().collect();

        let lower_bound = bounds[0] - 1;
        let upper_bound = bounds[1] - 1;

        if password[lower_bound as usize].to_string() == test_char
            && password[upper_bound as usize].to_string() != test_char
        {
            valid_count += 1;
        } else if password[lower_bound as usize].to_string() != test_char
            && password[upper_bound as usize].to_string() == test_char
        {
            valid_count += 1;
        }
    }

    println!("The number of valid passwords is {}", valid_count);
}

fn main() {
    let path = String::from("inputs/day_two/input.txt");
    let rows: Vec<String> = inputs::parse_input(path)
        .unwrap()
        .trim()
        .split('\n')
        .map(|contents| contents.parse().unwrap())
        .collect();

    part_one(&rows);
    part_two(&rows);
}
