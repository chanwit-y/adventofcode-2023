use crate::util;

pub struct DayOne;

impl DayOne {
    fn s2e(input: &str) -> char {
        let mut res: char = '-';
        for c in input.chars() {
            if c.is_digit(10) {
                res = c;
                break;
            }
        }
        res
    }

    fn e2s(input: &str) -> char {
        let mut res: char = '-';
        for c in input.chars().rev() {
            if c.is_digit(10) {
                res = c;
                break;
            }
        }
        res
    }

    fn index_s2e(input: &str) -> usize {
        let mut res: usize = 0;
        for (i, c) in input.chars().enumerate() {
            if c.is_digit(10) {
                res = i;
                break;
            }
        }
        res
    }

    fn index_e2s(input: &str) -> usize {
        let mut res: usize = 0;
        for (i, c) in input.chars().rev().enumerate() {
            if c.is_digit(10) {
                res = i;
                break;
            }
        }
        res
    }

    fn start_num(text: &str, end: usize) -> String {
	text[0..end].to_string()
    }

    fn end_num(text: &str, start: usize) -> String {
	text[start..].to_string()
    }

    pub fn run() {
        let input = util::file::Input::new("data/day1-1.txt");
        let vec = input.to_vec();

        let mut sum: i32 = 0;
        for v in vec {
            let mut digit = String::from("");

            // TODO: check char is not digit
            digit.push(DayOne::s2e(v));
            digit.push(DayOne::e2s(v));

            let digit = digit.parse::<i32>().unwrap();

            sum += digit;
        }

        println!("{}", sum);
    }

    pub fn run2() {
        let input = util::file::Input::new("data/day1-2.txt");
        let vec = input.to_vec();

	let d1 = DayOne::start_num(vec[0], DayOne::index_s2e(vec[0]));

	println!("{}", d1);
	
    }
}
