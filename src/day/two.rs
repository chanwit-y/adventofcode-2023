use crate::util;

pub struct DayTwo {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl DayTwo {
    //     fn split_group(&self, color: &str) -> (i32, &str) {
    //         (0, "")
    //     }

    pub fn new() -> DayTwo {
        DayTwo {
            red: 12,
            green: 13,
            blue: 14,
        }
    }

    fn process(&self, v: &str) -> (bool, i32) {
        let game: Vec<&str> = v.split(':').collect();
        let num = game[0].split(' ').collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        println!("game number: {:?}", num);

        let set = game[1].split(';');

        let mut is_over = false;
        for s in set.collect::<Vec<&str>>() {
            println!("set: {:?}", s);
            let colors = s.split(',').collect::<Vec<&str>>();
            for c in colors {
                let c = c.trim().split(' ').collect::<Vec<&str>>();

                println!("color: {:?}", c);
                if c[1] == "red" && c[0].parse::<i32>().unwrap() > self.red {
                    is_over = true;
                    println!("red: {:?}", self.red);
                } else if c[1] == "green" && c[0].parse::<i32>().unwrap() > self.green {
                    is_over = true;
                    println!("green: {:?}", self.green)
                } else if c[1] == "blue" && c[0].parse::<i32>().unwrap() > self.blue {
                    is_over = true;
                    println!("blue: {:?}", self.blue)
                }
            }

            //     if !is_break {
            // 	sum += num;
            //     }
        }

        // println!("is_over: {:?}", is_over);
        // if !is_over {
        //     sum += num;
        // }

        // println!("sum: {:?}", sum);

        (is_over, num)
    }

    pub fn run(&self) {
        let i = util::file::Input::new("data/day2-1.txt");
        let input = i.to_vec();
        let mut sum = 0;

	for v in input {
		let (is_over, id) = self.process(v);
		if !is_over {
			sum += id;
		}
		// println!("is_over: {:?}; id: {:?}", is_over, id);
	}
	println!("sum: {:?}", sum);

    }
}
