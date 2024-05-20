use std::fs;

const RED_LIMIT: i32 = 12;
const GREEN_LIMIT: i32 = 13;
const BLUE_LIMIT: i32 = 14;

fn main() {
    // Load input file 
    let input_path = "/Users/adomaitisc/Development/advent-of-code/2023/two/files/input";

    let contents = fs::read_to_string(input_path)
        .expect("could not open file");

    // log the file name
    println!("{}", input_path);

    let mut sum = 0;

    // log the file contents
    for (i, line) in contents.lines().enumerate() {

        //Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

        // steps to identify each game
        // index start at 1
        // split on : to get game results
        // split on ; to get each round results

        let game: Vec<&str> = line.split(":").collect();

        let results = game.get(1);

        if results.is_none() {
            println!("No results found for game {}", i + 1);
            continue;
        }

        let sets = results.unwrap().split(";");

        // let mut is_possible = true;

        let mut game_rgb: Vec<[i32; 3]> = Vec::new();

        for (j, set) in sets.enumerate() {

            let mut rgb = [0, 0, 0];

            let colors = set.split(",");
            for color in colors {
                // read first number and first letter
                let mut amount = String::new();

                for c in color.chars() {
                    if c.is_digit(10) {
                        amount.push(c)
                    } else if c == 'r' { // r...ed
                        rgb[0] = amount.parse::<i32>().unwrap();
                        // if rgb[0] > RED_LIMIT {
                        //     is_possible = false;
                        // }
                        break;
                    } else if c == 'g' { // g...reen
                        rgb[1] = amount.parse::<i32>().unwrap();
                        // if rgb[1] > GREEN_LIMIT {
                        //     is_possible = false;
                        // }
                        break;
                    } else if c == 'b' { // b...lue
                        rgb[2] = amount.parse::<i32>().unwrap();
                        // if rgb[2] > BLUE_LIMIT {
                        //     is_possible = false;
                        // }
                        break;
                    }
                }
            }

            game_rgb.push(rgb);
            println!("{:?}", rgb);
        }

        let mut min_cubes: [i32; 3] = [0, 0, 0];
        
        for rgb in game_rgb {
            for i in 0..3 {
                if min_cubes[i] < rgb[i] {
                    min_cubes[i] = rgb[i];
                }
            }
        }
        
        println!("Min cubes: {:?}", min_cubes);
        let cube_product = min_cubes[0] * min_cubes[1] * min_cubes[2];
        println!("Cube product: {}", cube_product);

        sum += cube_product;

    }
    println!("Sum: {}", sum);
}
