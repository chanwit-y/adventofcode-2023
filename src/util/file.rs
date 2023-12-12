use std::fs;

#[derive(Debug, Clone)]
pub struct Input(pub String);

impl Input {
    pub fn new(name: &str) -> Self {
        let content = fs::read_to_string(name).expect("Something went wrong reading the file");
        Input(content)
    }


    pub fn to_vec(&self) -> Vec<&str> {
	let res = self.0.split("\n").collect();
	res
    }
}
