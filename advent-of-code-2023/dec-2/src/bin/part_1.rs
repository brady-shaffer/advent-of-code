use std::fs;
use std::collections::HashMap;

fn main() {
    let filepath = "./input.txt";
    let game_data = read_input(&filepath);
    let game_number_sum = process_game_data(game_data);
    println!("Sum of game numbers: {}", game_number_sum);
}

fn read_input(filepath: &str) -> Vec<String> {
    let input = fs::read_to_string(filepath).unwrap();
    let mut output: Vec<String> = vec![];
    for line in input.lines() {
        output.push(String::from(line));
    }
    output
}

fn process_game_data(game_data: Vec<String>) -> i32 {
    let mut passing_games: Vec<i32> = vec![];

    let mut cube_limit_map = HashMap::new();
    cube_limit_map.insert(String::from("red"), 12);
    cube_limit_map.insert(String::from("green"), 13);
    cube_limit_map.insert(String::from("blue"), 14);

    for game in game_data {
        let game_split = game.split(":");
        let game_number = game_split.clone().collect::<Vec<&str>>()[0].split(" ").collect::<Vec<&str>>()[1];
        let rounds = game_split.collect::<Vec<&str>>()[1].split(";").collect::<Vec<&str>>();
        let mut game_passed = true;

        for round in rounds.iter() {
            let mut fail = false;
            let colors = round.split(",").collect::<Vec<&str>>();
            let mut cube_count = "0";
            for color in colors {
                let cube_color = color.trim().split(" ").collect::<Vec<&str>>()[1];
                cube_count = color.trim().split(" ").collect::<Vec<&str>>()[0];
                let limit = cube_limit_map.get(cube_color).unwrap().to_owned();
                if cube_count.parse::<i32>().unwrap() > limit && fail == false {
                    fail = true;
                    break;
                }
            }
            if fail == true {
                game_passed = false;
                break;
            }
        }
        if game_passed == true {
            passing_games.push(game_number.parse::<i32>().unwrap());
        }
    }

    passing_games.iter().sum::<i32>()
}