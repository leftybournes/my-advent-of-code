use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    let args: Vec<_> = env::args().collect();
    let path = &args[1];

    let mut calibration_values: Vec<u32> = vec![];

    let file = File::open(path).expect("Unable to open file from path");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let numbers: Vec<char> = String::from(line)
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .filter(|character| character.is_digit(10))
                .collect();

            let mut number_string = String::from(numbers[0]);
            number_string.push(numbers[numbers.len() - 1]);
            let number = number_string.parse::<u32>().expect("Not a number");
            calibration_values.push(number);
        }
    }

    let calibration_sum = calibration_values
        .into_iter()
        .reduce(|left, right| left + right);

    if let Some(sum) = calibration_sum {
        println!("Sum of calibration values: {sum}");
    }
}
