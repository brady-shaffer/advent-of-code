use std::fs;

fn main() {
    let filepath = "./input.txt";
    let output = read_input(&filepath);
    let first_and_last_digits = output.iter().map(|x| concat_first_and_last_chars(x)).collect::<Vec<i32>>();
    println!("{:?}", first_and_last_digits.iter().sum::<i32>());
}

fn read_input(filepath: &str) -> Vec<String> {
    let input = fs::read_to_string(filepath).unwrap();
    let mut output: Vec<String> = vec![];
    for line in input.lines() {
        output.push(String::from(line));
    }
    output
}

fn concat_first_and_last_chars(string: &String) -> i32 {
    let mut first_digit_flag = true;
    let mut first_digit = String::from("");
    let mut last_digit = String::from("");

    for c in string.chars() {
        if c.is_digit(10) && first_digit_flag {
            first_digit = c.to_string();
            first_digit_flag = false;
        }
        if c.is_digit(10) {
            last_digit = c.to_string();
        }
    }
    (first_digit + &last_digit).parse::<i32>().unwrap()
}