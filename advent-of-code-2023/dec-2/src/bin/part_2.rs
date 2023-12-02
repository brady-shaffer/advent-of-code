use std::fs;
use std::collections::HashMap;

fn main() {
    let filepath = "./input.txt";
    let game_data = read_input(&filepath);
    let game_number_sum = process_game_data(game_data);
    println!("Sum of all games: {}", game_number_sum);
}

fn read_input(filepath: &str) -> Vec<String> {
    let input = fs::read_to_string(filepath).unwrap();
    let mut output: Vec<String> = vec![];
    for line in input.lines() {
        output.push(String::from(line));
    }
    output
}

fn process_game_data(game_data: Vec<String>)  -> i32 {
    let mut cube_values_products = vec![];
    for game in game_data {
        let game_split = game.split(":");
        let rounds = game_split.collect::<Vec<&str>>()[1].split(";").collect::<Vec<&str>>();

        let mut cube_counts = HashMap::new();
        cube_counts.insert(String::from("red"), 0);
        cube_counts.insert(String::from("blue"), 0);
        cube_counts.insert(String::from("green"), 0);

        for round in rounds.iter() {
            let colors = round.split(",").collect::<Vec<&str>>();
            for color in colors {
                let cube_color = color.trim().split(" ").collect::<Vec<&str>>()[1];
                let cube_count = color.trim().split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
                if cube_count > cube_counts.get(cube_color).unwrap().to_owned() {
                    cube_counts.insert(String::from(cube_color), cube_count);
                }
            }
        }   
        let min_cube_values = cube_counts.into_values().collect::<Vec<i32>>();
        let values_product = min_cube_values[0] * min_cube_values[1] * min_cube_values[2];
        cube_values_products.push(values_product);
    }

    cube_values_products.iter().sum::<i32>()
}