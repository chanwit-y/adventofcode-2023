use crate::util;

pub struct DayOne;

impl DayOne {
    pub fn s2e(input: &str) -> (usize, char) {
        let mut res: (usize, char) = (0, '-');
        for c in input.chars().enumerate() {
            if c.1.is_digit(10) {
                res = c;
                break;
            }
        }
        res
    }

    fn e2s(input: &str) -> (usize, char) {
        let mut res: (usize, char) = (0, '-');
        for c in input.chars().rev().enumerate() {
            if c.1.is_digit(10) {
                res = c;
                break;
            }
        }
        res
    }

    pub fn run() {
        let input = util::file::Input::new("data/day1-1.txt");
        let vec = input.to_vec();

        let mut sum: i32 = 0;
        for v in vec {
            let mut digit = String::from("");

            // TODO: check char is not digit
            digit.push(DayOne::s2e(v).1);
            digit.push(DayOne::e2s(v).1);

            let digit = digit.parse::<i32>().unwrap();

            sum += digit;
        }

        println!("{}", sum);
    }
}

#[cfg(test)]
mod tests {
    //     use super::*;

    //     #[test]
    //     fn test_find_txt_num() {
    //         let input = "tlnllks2jcfdlgsjbhpfnineone";
    //         let res = DayOne::find_txt_num(input.to_string(), false);
    //         assert_eq!(res, (20, "nine"));
    //     }

    //     #[test]
    //     fn test_find_txt_num_revert() {
    //         let input = "tlnllks2jcfdlgsjbhpfnineone";
    //         let res = DayOne::find_txt_num(input.to_string(), true);
    //         assert_eq!(res, (24, "one"));
    //     }
}
