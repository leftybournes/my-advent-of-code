use my_advent_of_code::inputs;

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl Passport {
    fn new() -> Self {
        Passport {
            byr: String::from(""),
            iyr: String::from(""),
            eyr: String::from(""),
            hgt: String::from(""),
            hcl: String::from(""),
            ecl: String::from(""),
            pid: String::from(""),
            cid: String::from(""),
        }
    }
}

fn part_one() {
    let path = String::from("inputs/day_four/input.txt");
    let input = inputs::parse_input(path).expect("Error parsing passport data");

    let passports: Vec<String> = input
        .split("\n\n")
        .map(|passport| passport.parse().unwrap())
        .collect();

    let mut valid_passports = 0;

    for passport_data in passports.iter() {
        let passport_fields: Vec<String> = passport_data
            .split_whitespace()
            .map(|field| field.parse().unwrap())
            .collect();

        let mut passport = Passport::new();

        for field in passport_fields.iter() {
            let field_pair: Vec<String> =
                field.split(':').map(|pair| pair.parse().unwrap()).collect();

            match field_pair[0].as_str() {
                "byr" => passport.byr = field_pair[1].clone(),
                "iyr" => passport.iyr = field_pair[1].clone(),
                "eyr" => passport.eyr = field_pair[1].clone(),
                "hgt" => passport.hgt = field_pair[1].clone(),
                "hcl" => passport.hcl = field_pair[1].clone(),
                "ecl" => passport.ecl = field_pair[1].clone(),
                "pid" => passport.pid = field_pair[1].clone(),
                "cid" => passport.cid = field_pair[1].clone(),
                _ => println!("{} field not applicable", field_pair[0]),
            }
        }
        if !passport.byr.is_empty()
            && !passport.iyr.is_empty()
            && !passport.eyr.is_empty()
            && !passport.hgt.is_empty()
            && !passport.hcl.is_empty()
            && !passport.ecl.is_empty()
            && !passport.pid.is_empty()
        {
            valid_passports += 1;
        }
    }

    println!("Valid passports: {}", valid_passports);
}

fn main() {
    part_one();
}
