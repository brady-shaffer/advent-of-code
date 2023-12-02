use std::fs;
use std::collections::HashMap;
// use itertools::Itertools;

fn main() {
    let filepath = "./input.txt";
    let output = read_input(&filepath);
    let first_and_last_digits = output.iter().map(|x| index_digits(x)).collect::<Vec<i32>>();
    // for (i, line) in output.iter().enumerate() {
    //     println!("Line = {}, Decoded: {}", line, first_and_last_digits[i]);
    // }
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

fn parse_digit(string: &str) -> &str {
    match string {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => ""
    }
}

fn index_digits(string: &str) -> i32 {
    let string = string;
    let digits = vec![
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    let mut digit_indices_map: HashMap<usize, String> = HashMap::new();

    for digit in digits {
        if string.contains(digit) {
            let numeric_digit = parse_digit(digit);
            let digit_indices: Vec<(usize, &str)> = string.match_indices(digit).collect();
            for index in digit_indices {
                digit_indices_map.insert(index.0, numeric_digit.to_string());
            }
        }
    }

    for (i, c) in string.chars().enumerate() {
        if c.is_digit(10) {
            let c_str= c.to_string();
            digit_indices_map.insert(i, c_str);
        }
    }
    let first_digit = digit_indices_map.get(digit_indices_map.keys().min().unwrap()).unwrap().to_owned();
    let last_digit = digit_indices_map.get(digit_indices_map.keys().max().unwrap()).unwrap().to_owned();

    (first_digit + &last_digit).parse::<i32>().unwrap()
}