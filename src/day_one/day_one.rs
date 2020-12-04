const EXPECTED_NUM: i32 = 2020;

fn part_one(nums: &Vec<i32>) {
    let mut left_num = 0;
    let mut right_num = 0;

    let mut low_index = 0;
    let mut high_index = nums.len() - 1;

    while low_index < high_index {
        let sum = nums[low_index] + nums[high_index];

        if sum == EXPECTED_NUM {
            left_num = nums[low_index];
            right_num = nums[high_index];
            break;
        }

        if sum < EXPECTED_NUM {
            low_index += 1;
        } else {
            high_index -= 1;
        }
    }

    println!("First Number: {}; Second Number: {}", left_num, right_num);

    let product = left_num * right_num;
    println!("Multiply them and you get {}", product);
}

fn part_two(nums: &Vec<i32>) {
    let mut left_num = 0;
    let mut mid_num = 0;
    let mut right_num = 0;

    for num in nums.iter() {
        let subtracted = EXPECTED_NUM - num;

        let mut low_index = 0;
        let mut high_index = nums.len() - 1;

        let mut sum = 0;

        while low_index < high_index {
            sum = nums[low_index] + nums[high_index];

            if sum == subtracted {
                left_num = *num;
                mid_num = nums[low_index];
                right_num = nums[high_index];
                break;
            }

            if sum < subtracted {
                low_index += 1;
            } else {
                high_index -= 1;
            }
        }

        if sum == subtracted {
            break;
        }
    }

    println!(
        "First number: {}; Second Number: {}; Third Number: {}",
        left_num, mid_num, right_num
    );

    println!(
        "Multiply them and you get {}",
        left_num * mid_num * right_num
    );
}

use advent_of_code::inputs;

fn main() {
    let path = String::from("inputs/day_one/input.txt");
    let mut nums: Vec<i32> = inputs::provide_input(path)
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    nums.sort();

    part_one(&nums);
    part_two(&nums);
}
