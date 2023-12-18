use crate::util::{self};

pub struct DayThree;

impl DayThree {
    pub fn new() -> DayThree {
        DayThree
    }

    fn to_vec2d<'a>(&'a self, v: Vec<&'a str>) -> Vec<Vec<char>> {
        let mut vec2d = Vec::new();
        let mut line;

        for i in v {
            line = i.chars().collect::<Vec<char>>();
            vec2d.push(line);
        }

        vec2d
    }

    fn find_symbol(&self, table: Vec<Vec<char>>, i_r: usize, i_c: usize) -> (bool, char) {
        let symbot = ['*', '#', '$', '+', '@', '%', '&', '!', '/',  '|', '^', '~', '?', '>', '<', '(', ')', '[', ']', '{', '}', ';', ':', '"',  '`', '=', '-', '_', ' '];

        if i_r != 0 && i_c != 0 {
            let r_1 = i_r - 1;
            let c_1 = i_c - 1;
            if symbot.iter().find(|&&x| table[r_1][c_1] == x).is_some() {
                // println!("{} {}", true, table[r_1][c_1]);
                return (true, table[r_1][c_1]);
            }
        }

        if i_c != 0 {
            let r_2 = i_r;
            let c_2 = i_c - 1;
            if symbot.iter().find(|&&x| table[r_2][c_2] == x).is_some() {
                // println!("{} {}", true, table[r_2][c_2])
                return (true, table[r_2][c_2]);
            }
        }

        if i_r != table.len() - 1 && i_c != 0 {
            let r_3 = i_r + 1;
            let c_3 = i_c - 1;
            if symbot.iter().find(|&&x| table[r_3][c_3] == x).is_some() {
                // println!("{} {}", true, table[r_3][c_3])
                return (true, table[r_3][c_3]);
            }
        }

        if i_r != 0 {
            let r_4 = i_r - 1;
            let c_4 = i_c;
            if symbot.iter().find(|&&x| table[r_4][c_4] == x).is_some() {
                // println!("{} {}", true, table[r_4][c_4])
                return (true, table[r_4][c_4]);
            }
        }

        if i_r != table.len() - 1 {
            let r_5 = i_r + 1;
            let c_5 = i_c;
            if symbot.iter().find(|&&x| table[r_5][c_5] == x).is_some() {
                // println!("{} {}", true, table[r_5][c_5])
                return (true, table[r_5][c_5]);
            }
        }

        if i_r != 0 && i_c != table[0].len() - 1 {
            let r_6 = i_r - 1;
            let c_6 = i_c + 1;
            if symbot.iter().find(|&&x| table[r_6][c_6] == x).is_some() {
                // println!("{} {}", true, table[r_6][c_6])
                return (true, table[r_6][c_6]);
            }
        }

        if i_c != table[0].len() - 1 {
            let r_7 = i_r;
            let c_7 = i_c + 1;
            if symbot.iter().find(|&&x| table[r_7][c_7] == x).is_some() {
                // println!("{} {}", true, table[r_7][c_7])
                return (true, table[r_7][c_7]);
            }
        }

        if i_r != table.len() - 1 && i_c != table[0].len() - 1 {
            //     return (false, ' ');
            let r_8 = i_r + 1;
            let c_8 = i_c + 1;
            if symbot.iter().find(|&&x| table[r_8][c_8] == x).is_some() {
                // println!("{} {}", true, table[r_8][c_8])
                return (true, table[r_8][c_8]);
            }
        }

        (false, ' ')
    }


    pub fn run(&self) {
        let txt = util::file::Input::new("data/day3.txt");
        let input = txt.to_vec();
        let table = self.to_vec2d(input);
        let mut list_of_nums = Vec::new();
        for (i_r, r) in table.iter().enumerate() {
            let mut str = String::from("");
            let mut found = (false, ' ');
            for (i_c, c) in r.iter().enumerate() {
                if c.is_digit(10) && !found.0 {
                    found = self.find_symbol(table.clone(), i_r, i_c);
                }

                if c.is_digit(10) {
                    str.push(*c);
                }

                if str != "" && !c.is_digit(10) || i_c == r.len() - 1 {
                    if found.0 {
                        println!("num: {:?}, found: {:?}", str, found);
                        list_of_nums.push(str.clone());
                    }
                    found.0 = false;
                    str.clear();
                }
            }
        }

        let mut sum = 0;
        for i in list_of_nums {
            sum = sum + i.parse::<i32>().unwrap();
        }

        println!("{:?}", sum);
    }
}
