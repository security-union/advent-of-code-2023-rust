use std::{fs::read_to_string, path::Path};

use regex::Regex;

fn main() {
    let input = read_to_string(Path::new(
        "/Users/darioalessandro/Documents/advent-of-code-2023/day-5/src/input.txt",
    ))
    .unwrap();

    // parse input.
    let mut lines = input.lines();

    let number_regex = Regex::new(r"([0-9|])+").unwrap();

    let mut seeds = vec![];
    let mut seeds_iterator = lines.next().unwrap();

    // use regex to find all numbers in the line.
    let mut regex_results = number_regex.find_iter(seeds_iterator);

    // the values on the initial seeds: line come in pairs. Within each pair, the first value is the start of the range and the second value is the length of the range. So, in the first line of the example above:

    // seeds: 79 14 55 13
    // This line describes two ranges of seed numbers to be planted in the garden. The first range starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92. The second range starts with seed number 55 and contains 13 values: 55, 56, ..., 66, 67.
    let mut seed_range_start = regex_results.next().unwrap();
    let seed_range_length = regex_results.next().unwrap();
    // range 2
    let mut seed_range_start_2 = regex_results.next().unwrap();
    let seed_range_length_2 = regex_results.next().unwrap();

    // put all seeds in a vector.
    for seed in seed_range_start.as_str().parse::<u32>().unwrap()
        ..seed_range_start.as_str().parse::<u32>().unwrap()
            + seed_range_length.as_str().parse::<u32>().unwrap()
    {
        seeds.push(seed);
    }

    // println!("count seeds: {:?}", seeds.len());
    // println!("seeds: {:?}", seeds);

    // put all seeds in a vector.
    for seed in seed_range_start_2.as_str().parse::<u32>().unwrap()
        ..seed_range_start_2.as_str().parse::<u32>().unwrap()
            + seed_range_length_2.as_str().parse::<u32>().unwrap()
    {
        seeds.push(seed);
    }

    let mut seed_to_soil = vec![];
    let mut soil_to_fertilizer = vec![];
    let mut fertilizer_to_water = vec![];
    let mut water_to_light = vec![];
    let mut light_to_temperature = vec![];
    let mut temperature_to_humidity = vec![];
    let mut humidity_to_location = vec![];

    // use the label to determine which map to fill.
    let mut label_iterator = lines.next();

    while let Some(mut label) = label_iterator {
        while let Some(label_2) = label_iterator {
            if label_2.trim() != "" {
                label = label_2;
                break;
            }
            label_iterator = lines.next();
        }
        let mut map = vec![];
        while let Some(line) = lines.next() {
            if line.trim() == "" {
                break;
            }
            let mut regex_results = number_regex.find_iter(line);
            let destination_range_start = regex_results.next().unwrap();
            let source_range_start = regex_results.next().unwrap();
            let range_length = regex_results.next().unwrap();
            map.push(Mapper {
                destination_range_start: destination_range_start.as_str().parse::<u32>().unwrap(),
                source_range_start: source_range_start.as_str().parse::<u32>().unwrap(),
                range_length: range_length.as_str().parse::<u32>().unwrap(),
            });
        }
        match label.trim() {
            "seed-to-soil map:" => seed_to_soil = map,
            "soil-to-fertilizer map:" => soil_to_fertilizer = map,
            "fertilizer-to-water map:" => fertilizer_to_water = map,
            "water-to-light map:" => water_to_light = map,
            "light-to-temperature map:" => light_to_temperature = map,
            "temperature-to-humidity map:" => temperature_to_humidity = map,
            "humidity-to-location map:" => humidity_to_location = map,
            _ => panic!("unknown label: {}", label),
        }
        label_iterator = lines.next();
    }

    // print the maps.
    // println!("seeds: {:?}", seeds);
    // println!("seed-to-soil map: {:?}", seed_to_soil);
    // println!("soil-to-fertilizer map: {:?}", soil_to_fertilizer);
    // println!("fertilizer-to-water map: {:?}", fertilizer_to_water);
    // println!("water-to-light map: {:?}", water_to_light);
    // println!("light-to-temperature map: {:?}", light_to_temperature);
    // println!("temperature-to-humidity map: {:?}", temperature_to_humidity);
    // println!("humidity-to-location map: {:?}", humidity_to_location);

    // seed number 79 corresponds to soil number 81
    // seed number 14 corresponds to soil number 14
    // seed number 55 corresponds to soil number 57
    // seed number 13 corresponds to soil number 13

    // The first line has a destination range start of 50, a source range start of 98, and a range length of 2. This line means that the source range starts at 98 and contains two values: 98 and 99. The destination range is the same length, but it starts at 50, so its two values are 50 and 51. With this information, you know that seed number 98 corresponds to soil number 50 and that seed number 99 corresponds to soil number 51.
    fn mapper(mapper: &Vec<Mapper>, value: u32) -> u32 {
        let mut result = value;
        for map in mapper {
            if value >= map.source_range_start && value < map.source_range_start + map.range_length
            {
                result = map.destination_range_start + (value - map.source_range_start);
                break;
            }
        }
        result
    }
    // map seed to soil.
    let mut seed_to_soil_map = vec![];
    for seed in &seeds {
        seed_to_soil_map.push(mapper(&seed_to_soil, *seed));
    }

    // map soil to fertilizer.
    let mut soil_to_fertilizer_map = vec![];
    for soil in &seed_to_soil_map {
        soil_to_fertilizer_map.push(mapper(&soil_to_fertilizer, *soil));
    }

    // map fertilizer to water.
    let mut fertilizer_to_water_map = vec![];
    for fertilizer in &soil_to_fertilizer_map {
        fertilizer_to_water_map.push(mapper(&fertilizer_to_water, *fertilizer));
    }

    // map water to light.
    let mut water_to_light_map = vec![];
    for water in &fertilizer_to_water_map {
        water_to_light_map.push(mapper(&water_to_light, *water));
    }

    // map light to temperature.
    let mut light_to_temperature_map = vec![];
    for light in &water_to_light_map {
        light_to_temperature_map.push(mapper(&light_to_temperature, *light));
    }

    // map temperature to humidity.
    let mut temperature_to_humidity_map = vec![];
    for temperature in &light_to_temperature_map {
        temperature_to_humidity_map.push(mapper(&temperature_to_humidity, *temperature));
    }

    // map humidity to location.
    let mut humidity_to_location_map = vec![];
    for humidity in &temperature_to_humidity_map {
        humidity_to_location_map.push(mapper(&humidity_to_location, *humidity));
    }

    // find min
    humidity_to_location_map.sort();

    // print the maps.
    println!(
        "humidity_to_location_map: {:?}",
        humidity_to_location_map.first()
    );
}

#[derive(Debug)]
struct Mapper {
    pub destination_range_start: u32,
    pub source_range_start: u32,
    pub range_length: u32,
}

#[cfg(test)]
mod tests {

    use core::num;
    use std::{fs::read_to_string, path::Path};

    use regex::Regex;

    use super::*;

    #[test]
    fn test_map() {
        //         let input = "seeds: 79 14 55 13

        // seed-to-soil map:
        // 50 98 2
        // 52 50 48

        // soil-to-fertilizer map:
        // 0 15 37
        // 37 52 2
        // 39 0 15

        // fertilizer-to-water map:
        // 49 53 8
        // 0 11 42
        // 42 0 7
        // 57 7 4

        // water-to-light map:
        // 88 18 7
        // 18 25 70

        // light-to-temperature map:
        // 45 77 23
        // 81 45 19
        // 68 64 13

        // temperature-to-humidity map:
        // 0 69 1
        // 1 0 69

        // humidity-to-location map:
        // 60 56 37
        // 56 93 49";

        let input = read_to_string(Path::new(
            "/Users/darioalessandro/Documents/advent-of-code-2023/day-5/src/input.txt",
        ))
        .unwrap();

        // parse input.
        let mut lines = input.lines();

        let number_regex = Regex::new(r"([0-9|])+").unwrap();

        let mut seeds = vec![];
        let mut seeds_iterator = lines.next().unwrap();

        // use regex to find all numbers in the line.
        let mut regex_results = number_regex.find_iter(seeds_iterator);

        // the values on the initial seeds: line come in pairs. Within each pair, the first value is the start of the range and the second value is the length of the range. So, in the first line of the example above:

        // seeds: 79 14 55 13 34
        // This line describes two ranges of seed numbers to be planted in the garden. The first range starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92. The second range starts with seed number 55 and contains 13 values: 55, 56, ..., 66, 67.

        while regex_results.next().is_some() {
            let seed_range_start = regex_results.next().unwrap();
            let seed_range_length = regex_results.next().unwrap();
            // put all seeds in a vector.
            for seed in seed_range_start.as_str().parse::<u32>().unwrap()
                ..seed_range_start.as_str().parse::<u32>().unwrap()
                    + seed_range_length.as_str().parse::<u32>().unwrap()
            {
                seeds.push(seed);
            }
        }

        let mut seed_to_soil = vec![];
        let mut soil_to_fertilizer = vec![];
        let mut fertilizer_to_water = vec![];
        let mut water_to_light = vec![];
        let mut light_to_temperature = vec![];
        let mut temperature_to_humidity = vec![];
        let mut humidity_to_location = vec![];

        // use the label to determine which map to fill.
        let mut label_iterator = lines.next();

        while let Some(mut label) = label_iterator {
            while let Some(label_2) = label_iterator {
                if label_2.trim() != "" {
                    label = label_2;
                    break;
                }
                label_iterator = lines.next();
            }
            let mut map = vec![];
            while let Some(line) = lines.next() {
                if line.trim() == "" {
                    break;
                }
                let mut regex_results = number_regex.find_iter(line);
                let destination_range_start = regex_results.next().unwrap();
                let source_range_start = regex_results.next().unwrap();
                let range_length = regex_results.next().unwrap();
                map.push(Mapper {
                    destination_range_start: destination_range_start
                        .as_str()
                        .parse::<u32>()
                        .unwrap(),
                    source_range_start: source_range_start.as_str().parse::<u32>().unwrap(),
                    range_length: range_length.as_str().parse::<u32>().unwrap(),
                });
            }
            match label.trim() {
                "seed-to-soil map:" => seed_to_soil = map,
                "soil-to-fertilizer map:" => soil_to_fertilizer = map,
                "fertilizer-to-water map:" => fertilizer_to_water = map,
                "water-to-light map:" => water_to_light = map,
                "light-to-temperature map:" => light_to_temperature = map,
                "temperature-to-humidity map:" => temperature_to_humidity = map,
                "humidity-to-location map:" => humidity_to_location = map,
                _ => panic!("unknown label: {}", label),
            }
            label_iterator = lines.next();
        }

        // print the maps.
        // println!("seeds: {:?}", seeds);
        // println!("seed-to-soil map: {:?}", seed_to_soil);
        // println!("soil-to-fertilizer map: {:?}", soil_to_fertilizer);
        // println!("fertilizer-to-water map: {:?}", fertilizer_to_water);
        // println!("water-to-light map: {:?}", water_to_light);
        // println!("light-to-temperature map: {:?}", light_to_temperature);
        // println!("temperature-to-humidity map: {:?}", temperature_to_humidity);
        // println!("humidity-to-location map: {:?}", humidity_to_location);

        // seed number 79 corresponds to soil number 81
        // seed number 14 corresponds to soil number 14
        // seed number 55 corresponds to soil number 57
        // seed number 13 corresponds to soil number 13

        // The first line has a destination range start of 50, a source range start of 98, and a range length of 2. This line means that the source range starts at 98 and contains two values: 98 and 99. The destination range is the same length, but it starts at 50, so its two values are 50 and 51. With this information, you know that seed number 98 corresponds to soil number 50 and that seed number 99 corresponds to soil number 51.
        fn mapper(mapper: &Vec<Mapper>, value: u32) -> u32 {
            let mut result = value;
            for map in mapper {
                if value >= map.source_range_start
                    && value < map.source_range_start + map.range_length
                {
                    result = map.destination_range_start + (value - map.source_range_start);
                    break;
                }
            }
            result
        }
        // map seed to soil.
        let mut seed_to_soil_map = vec![];
        for seed in &seeds {
            seed_to_soil_map.push(mapper(&seed_to_soil, *seed));
        }

        // map soil to fertilizer.
        let mut soil_to_fertilizer_map = vec![];
        for soil in &seed_to_soil_map {
            soil_to_fertilizer_map.push(mapper(&soil_to_fertilizer, *soil));
        }

        // map fertilizer to water.
        let mut fertilizer_to_water_map = vec![];
        for fertilizer in &soil_to_fertilizer_map {
            fertilizer_to_water_map.push(mapper(&fertilizer_to_water, *fertilizer));
        }

        // map water to light.
        let mut water_to_light_map = vec![];
        for water in &fertilizer_to_water_map {
            water_to_light_map.push(mapper(&water_to_light, *water));
        }

        // map light to temperature.
        let mut light_to_temperature_map = vec![];
        for light in &water_to_light_map {
            light_to_temperature_map.push(mapper(&light_to_temperature, *light));
        }

        // map temperature to humidity.
        let mut temperature_to_humidity_map = vec![];
        for temperature in &light_to_temperature_map {
            temperature_to_humidity_map.push(mapper(&temperature_to_humidity, *temperature));
        }

        // map humidity to location.
        let mut humidity_to_location_map = vec![];
        for humidity in &temperature_to_humidity_map {
            humidity_to_location_map.push(mapper(&humidity_to_location, *humidity));
        }

        // find min
        humidity_to_location_map.sort();

        // print the maps.
        println!(
            "humidity_to_location_map: {:?}",
            humidity_to_location_map.first()
        );
    }
}
