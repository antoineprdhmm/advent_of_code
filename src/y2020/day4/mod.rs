use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::read_input;

const BIRTH_YEAR_KEY: &str = "byr";
const ISSUE_YEAR_KEY: &str = "iyr";
const EXPIRATION_YEAR_KEY: &str = "eyr";
const HEIGHT_KEY: &str = "hgt";
const HAIR_COLOR_KEY: &str = "hcl";
const EYE_COLOR_KEY: &str = "ecl";
const PASSPORT_ID_KEY: &str = "pid";

#[derive(Default, Debug)]
struct Passport(HashMap<String, String>);

impl Passport {
    pub fn have_all_required_fields(&self) -> bool {
        self.0.contains_key(BIRTH_YEAR_KEY)
            && self.0.contains_key(ISSUE_YEAR_KEY)
            && self.0.contains_key(EXPIRATION_YEAR_KEY)
            && self.0.contains_key(HEIGHT_KEY)
            && self.0.contains_key(EYE_COLOR_KEY)
            && self.0.contains_key(PASSPORT_ID_KEY)
            && self.0.contains_key(HAIR_COLOR_KEY)
    }

    fn is_date_value_between_dates(
        &self,
        date_value_option: &'static str,
        from: usize,
        to: usize,
    ) -> bool {
        match self.0.get(date_value_option) {
            Some(date_value) => {
                let date: usize = date_value.parse().unwrap();
                date >= from && date <= to
            }
            None => false,
        }
    }

    fn is_height_valid(&self) -> bool {
        match self.0.get(HEIGHT_KEY) {
            Some(height) => {
                let mut height = height.clone();

                let unit = height.pop().unwrap();
                height.pop();

                let number: usize = height.parse().unwrap();

                if unit == 'm' {
                    return (150..=193).contains(&number);
                }

                (59..=76).contains(&number)
            }
            None => false,
        }
    }

    fn is_hair_color_valid(&self) -> bool {
        let re = Regex::new(r"^#[\da-f]{6}$").unwrap();

        match self.0.get(HAIR_COLOR_KEY) {
            Some(hair_color) => re.is_match(hair_color),
            None => false,
        }
    }

    fn is_eye_color_valid(&self) -> bool {
        let eye_colors: HashSet<&'static str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .cloned()
            .collect();

        match self.0.get(EYE_COLOR_KEY) {
            Some(eye_color) => eye_colors.contains(eye_color.as_str()),
            None => false,
        }
    }

    fn is_passport_id_valid(&self) -> bool {
        let re = Regex::new(r"^\d{9}$").unwrap();

        match self.0.get(PASSPORT_ID_KEY) {
            Some(passport_id) => re.is_match(passport_id),
            None => false,
        }
    }

    fn is_birth_year_valid(&self) -> bool {
        self.is_date_value_between_dates(BIRTH_YEAR_KEY, 1920, 2002)
    }

    fn is_issue_year_valid(&self) -> bool {
        self.is_date_value_between_dates(ISSUE_YEAR_KEY, 2010, 2020)
    }

    fn is_expiration_year_valid(&self) -> bool {
        self.is_date_value_between_dates(EXPIRATION_YEAR_KEY, 2020, 2030)
    }

    pub fn is_passport_valid(&self) -> bool {
        self.is_birth_year_valid()
            && self.is_issue_year_valid()
            && self.is_expiration_year_valid()
            && self.is_height_valid()
            && self.is_hair_color_valid()
            && self.is_eye_color_valid()
            && self.is_passport_id_valid()
    }
}

fn read_passports() -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::from([Default::default()]);

    read_input(2020, 4)
        .unwrap()
        .map_while(Result::ok)
        .for_each(|line| {
            if line.is_empty() {
                passports.push(Default::default());
            } else {
                let passport = passports.last_mut();
                if let Some(passport) = passport {
                    line.split(" ")
                        .map(|key_value| key_value.split_once(":").unwrap())
                        .for_each(|(key, value)| {
                            passport.0.insert(key.to_string(), value.to_string());
                        });
                }
            }
        });

    passports
}

pub fn run_part_1() {
    let passports = read_passports();
    let count = passports
        .iter()
        .filter(|passport| passport.have_all_required_fields())
        .count();
    assert_eq!(count, 206);
}

pub fn run_part_2() {
    let passports = read_passports();
    let count = passports
        .iter()
        .filter(|passport| passport.is_passport_valid())
        .count();
    assert_eq!(count, 123);
}
