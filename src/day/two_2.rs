use crate::util;

pub struct DayTwo2 {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl DayTwo2 {
    //     fn split_group(&self, color: &str) -> (i32, &str) {
    //         (0, "")
    //     }

    pub fn new() -> DayTwo2 {
        DayTwo2 {
            red: 12,
            green: 13,
            blue: 14,
        }
    }

    fn process(&self, v: &str) -> i32  {
	let mut max_red = 1;
	let mut max_green = 1;
	let mut max_blue = 1;

        let game: Vec<&str> = v.split(':').collect();
        // let num = game[0].split(' ').collect::<Vec<&str>>()[1]
        //     .parse::<i32>()
        //     .unwrap();
        // println!("game number: {:?}", num);

        let set = game[1].split(';');

        // let mut is_over = false;
        for s in set.collect::<Vec<&str>>() {
        //     println!("set: {:?}", s);
            let colors = s.split(',').collect::<Vec<&str>>();
            for c in colors {
                let c = c.trim().split(' ').collect::<Vec<&str>>();

                // println!("color: {:?}", c);
                if c[1] == "red"  && c[0].parse::<i32>().unwrap() > max_red   {
			max_red = c[0].parse::<i32>().unwrap();
                } else if c[1] == "green" && c[0].parse::<i32>().unwrap() > max_green   {
			max_green = c[0].parse::<i32>().unwrap();
                } else if c[1] == "blue" && c[0].parse::<i32>().unwrap() > max_blue   {
			max_blue = c[0].parse::<i32>().unwrap();
                }
            }
        }

	// println!("max_red: {:?}", max_red);
	// println!("max_green: {:?}", max_green);
	// println!("max_blue: {:?}", max_blue);
	// 0
	max_red * max_green * max_blue
        // (is_over, num)
    }

    pub fn run(&self) {
        let i = util::file::Input::new("data/day2-1.txt");
        let input = i.to_vec();
        let mut sum = 0;

	for v in input {
		let res =  self.process(v);
		sum += res;
		// let (is_over, id) = self.process(v);
		// if !is_over {
		// 	sum += id;
		// }
	}
	println!("sum: {:?}", sum);

    }
}
