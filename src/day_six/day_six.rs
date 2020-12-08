use my_advent_of_code::inputs;

// the number of letters in the english alphabet (a-z)
const ALPHA_SIZE: usize = 26;

fn part_one(input: &Vec<String>) -> u32 {
    let base_digit = 'a' as usize;
    let mut count_sum = 0;

    for answers in input.iter() {
        let mut letterarr = vec![false; ALPHA_SIZE];
        let letters: String = answers
            .split_whitespace()
            .map(|letter| letter.to_string())
            .collect();

        for letter in letters.chars() {
            let letter_as_digit = letter as usize;
            letterarr[letter_as_digit - base_digit] = true;
        }

        for is_answered in letterarr.into_iter() {
            if is_answered {
                count_sum += 1
            }
        }
    }

    count_sum
}

fn part_two(input: &Vec<String>) -> u32 {
    let base_digit = 'a' as usize;
    let mut count_sum = 0;

    for people in input.iter() {
        let mut letterarr = vec![0; ALPHA_SIZE];
        let answers: Vec<String> = people.lines().map(|letter| letter.to_string()).collect();
        let people_count = answers.len();

        for answer in answers.iter() {
            for letter in answer.chars() {
                let letter_as_digit = letter as usize;
                letterarr[letter_as_digit - base_digit] += 1;
            }
        }

        for answer_count in letterarr.into_iter() {
            if answer_count == people_count {
                count_sum += 1
            }
        }
    }

    count_sum
}

fn main() {
    let path = String::from("inputs/day_six/input.txt");
    let input: Vec<String> = inputs::parse_input(path)
        .expect("Error parsing answers")
        .split("\n\n")
        .map(|row| row.parse().unwrap())
        .collect();

    let count_sum = part_one(&input);
    println!("Sum of count part one: {}", count_sum);

    let count_sum_two = part_two(&input);
    println!("Sum of count part two: {}", count_sum_two);
}
