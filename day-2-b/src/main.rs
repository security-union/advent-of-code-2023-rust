use std::{collections::HashMap, fs::read_to_string, path::Path};

fn main() {
    // load input file
    let input = read_to_string(Path::new(
        "/Users/darioalessandro/Documents/advent-of-code-2023/day-2/src/input.txt",
    ))
    .unwrap();

    let find_min: Vec<_> = find_min_cubes(&input);


    let actual_power: u16 = find_min.iter().fold(0, |acc, hash_map| {
        let product: u16 = hash_map.values().product();
        acc + product
    });
    
    println!("sum {}", actual_power);
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

fn find_min_cubes(input: &str) -> Vec<HashMap<&str, u16>> {
    // parse input into games
    let games: Vec<Game> = input.split('\n').map(game_parser).collect();
    println!("games count {}", games.len());
    games.into_iter().map(|game| {
        let mut balls = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ]);

        for round in game.rounds {
            for ball_color in round.keys() {
                let ball_count_for_color = round.get(ball_color).unwrap();
                if ball_count_for_color > balls.get(ball_color).unwrap() {
                    balls.insert(ball_color, *ball_count_for_color);
                }
            }
        }
        balls
    }).collect()
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
    fn test_all_combinations() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let expected_number_of_cubes = vec![
            HashMap::from([
                ("red", 4u16),
                ("green", 2u16),
                ("blue", 6u16),
            ]),
            HashMap::from([
                ("red", 1u16),
                ("green", 3u16),
                ("blue", 4u16),
            ]),
            HashMap::from([
                ("red", 20u16),
                ("green", 13u16),
                ("blue", 6u16),
            ]),
            HashMap::from([
                ("red", 14u16),
                ("green", 3u16),
                ("blue", 15u16),
            ]),
            HashMap::from([
                ("red", 6u16),
                ("green", 3u16),
                ("blue", 2u16),
            ])
        ];

        // find the min # of green, blue and red cubes to complete each game.
        let find_min: Vec<_> = find_min_cubes(&input);

        assert_eq!(expected_number_of_cubes, find_min);

        // We need to multiply all ball values 
        let expected_power = 2286;

        let actual_power: u16 = find_min.iter().fold(0, |acc, hash_map| {
            let product: u16 = hash_map.values().product();
            acc + product
        });

        assert_eq!(expected_power, actual_power);


    } 
}
