use std::{collections::HashMap, fs::read_to_string, path::Path};

fn main() {
    // load input file
    let input_file = read_to_string(Path::new(
        "/Users/darioalessandro/Documents/advent-of-code-2023/day-2/src/input.txt",
    ))
    .unwrap();

    let available_cubes = HashMap::from([
        ("red", 12u16),
        ("green", 13u16),
        ("blue", 14u16),
    ]);

    let game_ids = find_game_ids(&input_file, &available_cubes);
    // sum ids
    println!("game ids {:?}", game_ids);
    let sum: u16 = game_ids.iter().sum();
    println!("sum {}", sum);
}

fn find_game_ids(input: &str, available_cubes: &HashMap<&str, u16>) -> Vec<u16> {
    let games: Vec<Game> = input.split('\n').map(game_parser).collect();
    println!("games count {}", games.len());
    games.into_iter().filter_map(|game| {
        for round in game.rounds {
            let  available_cubes = available_cubes.clone();
            // attempt to pull the cubes, if we fail return None
            // Iterate through hashmap
            for color in round.keys() {
                let cubes_of_color = round.get(color).unwrap();
                // check if we have enough cubes of color in available_cubes
                if let Some(available_cubes_of_color)  = available_cubes.get(color) {
                    if available_cubes_of_color >= cubes_of_color {
                        // do nothing
                    } else {
                        return  None;
                    }
                } else {
                    return None;
                }
            }
        }
        println!("available_cubes {:?}", available_cubes);
        Some(game.id)
    }).collect()
}

#[derive(Default, PartialEq, Debug)]
struct Game<'a> {
    id: u16,
    rounds: Vec<HashMap<&'a str, u16>>
}

fn game_parser(input :&str) -> Game {
    // "Game x: round 1; round 2"
    // split on semicolon to find game id
    let mut chunks = input.split(':');
    // first chunk contains the id: Game xxx
    // replace "Game " with ""
    let game_id = chunks.next().unwrap().replace("Game ", "");
    let game_id = u16::from_str_radix(&game_id, 10).unwrap();

    // next chunk has all the rounds
    //  3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let rounds = chunks.next().unwrap().split(";").map(|round| {
        // 3 blue, 4 red
        // split on ,
        let round: HashMap<&str, u16> = round.split(',').map(|cubes_string| {
            //n color
            let mut cubes = cubes_string.split(" ");
            let _empty_chunk = cubes.next();
            let number_of_cubes = cubes.next().unwrap().parse::<u16>().unwrap();
            let color = cubes.next().unwrap();
            (color, number_of_cubes)
        }).collect();
        round
    }).collect();

    Game {
        id: game_id, 
        rounds
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap};

    use super::*;

    #[test]
    fn test_game_parser() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected_output = Game {
            id: 1,
            rounds: vec!(
                HashMap::from([
                    ("blue", 3u16),
                    ("red", 4u16),
                ]),
                HashMap::from([
                    ("red", 1),
                    ("green", 2),
                    ("blue", 6)
                ]),
                HashMap::from([
                    ("green", 2)
                ])
            )
        };
        let actual_output = game_parser(&input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_find_error() {
        let input = "Game 81: 1 green, 5 blue; 1 green, 3 blue, 1 red; 1 blue, 1 red, 3 green; 6 blue, 5 green";
        let available_cubes = HashMap::from([
            ("red", 12u16),
            ("green", 13u16),
            ("blue", 14u16),
        ]);
        let actual_cubes = find_game_ids(&input, &available_cubes);
        assert_eq!(vec!(81u16), actual_cubes);

    }
    #[test]
    fn test_all_combinations() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let expected_game_ids = vec![1,2, 5];
        let expected_sum = 8;

        let available_cubes = HashMap::from([
            ("red", 12u16),
            ("green", 13u16),
            ("blue", 14u16),
        ]);

        let actual_game_ids = find_game_ids(&input, &available_cubes);
        let actual_sum: u16 = actual_game_ids.iter().sum();
        assert_eq!(expected_game_ids, actual_game_ids);
        assert_eq!(expected_sum, actual_sum);
    } 
}
