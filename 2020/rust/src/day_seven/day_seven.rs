use my_advent_of_code::inputs;

struct BagRule {
    outer_bag: String,
    contents: Vec<Bag>,
}

impl BagRule {
    fn new() -> BagRule {
        BagRule {
            outer_bag: String::new(),
            contents: Vec::new(),
        }
    }
}

struct Bag {
    amount: u32,
    description: String,
}

impl Bag {
    fn from(amount: u32, description: &String) -> Bag {
        Bag {
            amount,
            description: String::from(description),
        }
    }
}

fn part_one(input: &String) -> u32 {
    // let mut bag_rules =

    let rules: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let mut bag_rules: Vec<BagRule> = Vec::new();

    for rule in &rules {
        let description: Vec<String> = rule
            .split("contain")
            .map(|description| description.to_string())
            .collect();

        let outer_bag = description.get(0);
        let inner_bags = description.get(1);

        let mut bag_rule = BagRule::new();

        if let Some(bag) = outer_bag {
            if bag.contains("bags") {
                bag_rule.outer_bag = bag.replace("bags", "").trim().to_string();
            } else if bag.contains("bag") {
                bag_rule.outer_bag = bag.replace("bag", "").trim().to_string();
            }
        }

        if let Some(bag) = inner_bags {
            let bags: Vec<String> = bag
                .split(',')
                .map(|description| description.to_string())
                .collect();

            for bag_description in &bags {
                if (!bag_description.contains("no other bags")) {
                    let amount = bag_description
                        .chars()
                        .filter(|c| c.is_digit(10))
                        .collect::<String>()
                        .parse::<u32>();

                    let mut description = bag_description
                        .chars()
                        .filter(|c| !c.is_digit(10))
                        .collect::<String>()
                        .replace('.', "");

                    if description.contains("bags") {
                        description = description.replace("bags", "").trim().to_string();
                    } else if description.contains("bag") {
                        description = description.replace("bag", "").trim().to_string();
                    }

                    match amount {
                        Err(why) => println!("{:?}", why),
                        Ok(parsed_amount) => {
                            let bag = Bag::from(parsed_amount, &description);

                            bag_rule.contents.push(bag);
                        }
                    }
                }
            }
        }

        println!("Bag: {}", bag_rule.outer_bag);
        print!("Contents: ");
        for bag in bag_rule.contents.iter() {
            println!("{}", bag.description);
        }
    }

    0
}

fn main() {
    let path = String::from("inputs/day_seven/sample_input.txt");
    let input = inputs::parse_input(path).expect("Unable to parse bag rules.");

    let amount_of_bags_containing_shiny_gold = part_one(&input);
    println!(
        "Amount of bags containing at least 1 shiny gold bag: {}",
        amount_of_bags_containing_shiny_gold
    );
}
