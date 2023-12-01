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

    fn from_string_data(passport_fields: &Vec<String>) -> Self {
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

        passport
    }

    fn validate(&self) -> bool {
        self.validate_byr()
            && self.validate_iyr()
            && self.validate_eyr()
            && self.validate_hgt()
            && self.validate_hcl()
            && self.validate_ecl()
            && self.validate_pid()
    }

    fn validate_byr(&self) -> bool {
        if self.byr.is_empty() {
            return false;
        }

        let byr_len = self.byr.len();

        let year = self.byr.parse::<u32>().unwrap();

        byr_len == 4 && (1920..=2002).contains(&year)
    }

    fn validate_iyr(&self) -> bool {
        if self.iyr.is_empty() {
            return false;
        }

        let iyr_len = self.iyr.len();

        let year = self.iyr.parse::<u32>().unwrap();

        iyr_len == 4 && (2010..=2020).contains(&year)
    }

    fn validate_eyr(&self) -> bool {
        if self.eyr.is_empty() {
            return false;
        }

        let eyr_len = self.eyr.len();

        let year = self.eyr.parse::<u32>().unwrap();

        eyr_len == 4 && (2020..=2030).contains(&year)
    }

    fn validate_hgt(&self) -> bool {
        if self.hgt.is_empty() {
            return false;
        }

        let height = self
            .hgt
            .chars()
            .filter(|c| ('0'..='9').contains(&c))
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        (self.hgt.contains("in") && (59..=76).contains(&height))
            || (self.hgt.contains("cm") && (150..=193).contains(&height))
    }

    fn validate_hcl(&self) -> bool {
        if self.hcl.is_empty() {
            return false;
        }

        let hcl_len = self.hcl.len();
        let first_char = &self.hcl[0..1];
        let clr_code = &self.hcl[1..];

        hcl_len == 7
            && first_char == "#"
            && clr_code.chars().fold(true, |acc, c| {
                acc && (('0'..='9').contains(&c)
                    || ('a'..='f').contains(&c)
                    || ('A'..='F').contains(&c))
            })
    }

    fn validate_ecl(&self) -> bool {
        if self.ecl.is_empty() {
            return false;
        }

        self.ecl == "amb"
            || self.ecl == "blu"
            || self.ecl == "brn"
            || self.ecl == "gry"
            || self.ecl == "grn"
            || self.ecl == "hzl"
            || self.ecl == "oth"
    }

    fn validate_pid(&self) -> bool {
        if self.pid.is_empty() {
            return false;
        }

        self.pid.len() == 9
    }
}

fn part_one(passports: &Vec<String>) -> u32 {
    let mut valid_passports: u32 = 0;

    for passport_data in passports.iter() {
        let passport_fields: Vec<String> = passport_data
            .split_whitespace()
            .map(|field| field.parse().unwrap())
            .collect();

        let passport = Passport::from_string_data(&passport_fields);

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

    valid_passports
}

fn part_two(passports: &Vec<String>) -> u32 {
    let mut valid_passports: u32 = 0;

    for passport_data in passports.iter() {
        let passport_fields: Vec<String> = passport_data
            .split_whitespace()
            .map(|field| field.parse().unwrap())
            .collect();

        let passport = Passport::from_string_data(&passport_fields);

        if passport.validate() {
            valid_passports += 1;
        }
    }

    valid_passports
}

fn main() {
    let path = String::from("inputs/day_four/input.txt");
    let input = inputs::parse_input(path).expect("Error parsing passport data");

    let passports: Vec<String> = input
        .split("\n\n")
        .map(|passport| passport.parse().unwrap())
        .collect();

    let pone_valid_passports = part_one(&passports);
    println!("Valid passports part one: {}", pone_valid_passports);

    let ptwo_valid_passports = part_two(&passports);
    println!("Valid passports part two: {}", ptwo_valid_passports);
}
