use my_advent_of_code::inputs;

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    // The challenge specifies a cid field but it's optional so we won't check
    // for that.
}
