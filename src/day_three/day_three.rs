use my_advent_of_code::inputs;

struct Slope {
    horizontal_step: u32,
    vertical_step: u32,
}

fn count_trees(slope: &Slope) -> u32 {
    let path = String::from("inputs/day_three/input.txt");
    let input = inputs::parse_input(path).expect("Error parsing map input");

    let field_map: Vec<String> = input.lines().map(|row| row.parse().unwrap()).collect();
    let field_width = field_map[0].len();

    let mut tree_count = 0;
    let mut horizontal_progress: u32 = 0;
    let mut vertical_progress: u32 = 0;

    let vertical_limit = field_map.len() - 1;

    while vertical_progress < vertical_limit as u32 {
        horizontal_progress += slope.horizontal_step;
        vertical_progress += slope.vertical_step;

        if horizontal_progress >= field_width as u32 {
            horizontal_progress %= field_width as u32;
        }

        if field_map[vertical_progress as usize]
            .chars()
            .collect::<Vec<char>>()[horizontal_progress as usize]
            == '#'
        {
            tree_count += 1;
        }
    }

    tree_count
}

fn part_one() -> u32 {
    let slope = Slope {
        horizontal_step: 3,
        vertical_step: 1,
    };

    count_trees(&slope)
}

fn part_two() -> u32 {
    let slopes = vec![
        Slope {
            horizontal_step: 1,
            vertical_step: 1,
        },
        Slope {
            horizontal_step: 3,
            vertical_step: 1,
        },
        Slope {
            horizontal_step: 5,
            vertical_step: 1,
        },
        Slope {
            horizontal_step: 7,
            vertical_step: 1,
        },
        Slope {
            horizontal_step: 1,
            vertical_step: 2,
        },
    ];

    let mut tree_product = 1;

    for slope in slopes.iter() {
        tree_product *= count_trees(slope);
    }

    tree_product
}

fn main() {
    println!("Part One answer {}", part_one());
    println!("Part Two answer {}", part_two());
}
