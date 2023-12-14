use crate::{util, day::one::DayOne};

pub struct DayOne2;

impl DayOne2 {
fn find_txt_num<'a>(text: String, is_revert: bool) -> (usize, &'a str) {
        let nums = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let mut res = (999, "");

        for n in nums {
            if is_revert {
                let i = match text.rfind(n) {
                    Some(x) => x,
                    None => 0,
                };

		// println!("i: {:?}", i);
		// println!("res: {:?}", res);

                if res.0 == 999 || i > res.0 {
                    let num = DayOne2::check_text_num(n.to_string());
                    res = (i, num);
                }
            } else {
                let i = match text.find(n) {
                    Some(x) => x,
                    None => 999,
                };

		// println!("text: {} n: {} i: {:?}", text, n, i);
		// println!("res: {:?}", res);

                if (res.0 == 999 || i < res.0)  && i != 999 {
                    let num = DayOne2::check_text_num(n.to_string());
                    res = (i, num);
                }
		println!("res: {:?}", res);
            }

            //     let i = text.find(n);
            //     match i {
            //         Some(i) => {
            //             if is_revert {
            //                 if res.0 == 999 || res.0 < i {
            //                     let num = DayOne::check_text_num(n.to_string());
            //                     //     let index = text.find(n);
            //                     //     println!("index: {:?}", index);
            //                     // get last index
            //                     let index = match text.rfind(n) {
            //                         Some(x) => x,
            //                         None => 0,
            //                     };
            //                     //     println!("n: {:?} index: {:?}", n, index);
            //                     // println!("index: {:?}", index);
            //                     // sevenseven482seven
            //                     res = (index, num);
            //                 }
            //             } else {
            //                 if res.0 == 999 || res.0 > i {
            //                     //     res = (i, n);
            //                     let num = DayOne::check_text_num(n.to_string());
            //                     res = (i, num);
            //                 }
            //             }
            //         }
            //         None => {} // None => println!("{} is not found", n),
            //     }
        }

        println!("res: {:?}", res);
        res
    }

    fn check_text_num<'a>(text: String) -> &'a str {
        // let mut res = "";
        // let mut res = String::from("");
        let res = match text.as_str() {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => "0",
        };

        res
    }

     fn e2s(input: &str) -> (usize, char) {
        let mut res: (usize, char) = (0, '-');
        for c in input.chars().rev().enumerate() {
            if c.1.is_digit(10) {
                // println!("c: {:?}", input.find(c.1));
                let x = match input.rfind(c.1) {
                    Some(x) => x,
                    None => 0,
                };
                res = (x, c.1);
                break;
            }
        }
        res
    }

    fn get_num(v: &str) -> i32 {
        let s_digit = DayOne::s2e(v);
        let e_digit = DayOne2::e2s(v);
        let s_txt = DayOne2::find_txt_num(v.to_string(), false);
        let e_txt = DayOne2::find_txt_num(v.to_string(), true);

        println!("s_digit: {:?}", s_digit);
        println!("s_txt: {:?}", s_txt);

        println!("e_digit: {:?}", e_digit);
        println!("e_txt: {:?}", e_txt);


        let mut digit = String::from("");
        digit.push(
            if s_digit.0 <= s_txt.0 && s_digit.1 != '-' || s_txt.0 == 999 {
                s_digit.1
            } else {
                s_txt.1.chars().next().unwrap()
            },
        );

        digit.push(
            //TODO: FIX
            if e_digit.0 >= e_txt.0 && e_digit.1 != '-' || e_txt.0 == 999 {
                e_digit.1
            } else {
                e_txt.1.chars().next().unwrap()
                // match e_txt.1.chars().next() {
                //     Some(x) => x,
                //     None => {
                //         println!("e_digit: {:?}", e_digit);
                //         println!("e_txt: {:?}", e_txt);
                //         '-'
                //     }
                // }

                //Fixme: this is not correct
            },
        );

        println!("digit: {:?}", digit);

        digit.parse::<i32>().unwrap()
    }

    pub fn run() {
        let input = util::file::Input::new("data/day1-2.txt");
        let vec = input.to_vec();
        let mut sum = 0;

        for v in vec {
            let num = DayOne2::get_num(v);
            println!("text: {} num: {:?}", v, num);
            sum += num;
        }

        println!("sum: {:?}", sum);

        // let s_digit = DayOne::s2e(vec[2]);
        // let e_digit = DayOne::e2s_2(vec[2]);
        // let s_txt = DayOne::find_txt_num(vec[2].to_string(), false);
        // let e_txt = DayOne::find_txt_num(vec[2].to_string(), true);

        // let mut digit = String::from("");
        // digit.push(if s_digit.0 <= s_txt.0 {s_digit.1} else {s_txt.1.chars().next().unwrap()});
        // digit.push(if e_digit.0 >= e_txt.0 {e_digit.1} else {e_txt.1.chars().next().unwrap()});

        // println!("s_digit: {:?}", s_digit);
        // println!("s_txt: {:?}", s_txt);

        // println!("e_digit: {:?}", e_digit);
        // println!("e_txt: {:?}", e_txt);

        // println!("digit: {:?}", digit);
    }
}