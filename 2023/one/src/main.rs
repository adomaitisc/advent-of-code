use std::fs;

const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn get_first_number(line: &str) -> i32 {
    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            // if the character is a number, we return it parsed
            return c.to_string().parse::<i32>().unwrap();

        } else if c.is_alphabetic() {
            // grab the remaining line to compare with the numbers
            let inclusive_remaining = &line[i..];

            for (j, num) in NUMBERS.iter().enumerate() {
                // if the remaining line starts with a number, we return the index + 1
                if inclusive_remaining.starts_with(num) { return (j + 1) as i32; }
            }
        }
    }

    0
}

fn get_last_number(line: &str) -> i32 {
    // create a reversed copy of the line to iterate through
    let rev_line = line.chars().rev().collect::<String>();

    for (i, c) in rev_line.chars().enumerate() {
        if c.is_numeric() {
            // if the character is a number, we return it parsed
            return c.to_string().parse::<i32>().unwrap();

        } else if c.is_alphabetic() {
            // grab the remaining line to compare with the numbers
            let inclusive_remaining = &rev_line[i..];

            for (j, num) in NUMBERS.iter().enumerate() {
                // reverse the number to compare with the remaining line
                let rev_num = num.chars().rev().collect::<String>();

                // if the remaining line starts with a number, we return the index + 1
                if inclusive_remaining.starts_with(rev_num.as_str()) { return (j + 1) as i32; }
            }
        }
    }

    0
}

fn main() {
    // Load input file 
    let input_path = "/Users/adomaitisc/Development/advent-of-code/2023/one/input";

    let contents = fs::read_to_string(input_path)
        .expect("could not open file");

    // log the file name
    println!("{}", input_path);


    let start = std::time::Instant::now();
    let mut sum = 0;

    for line in contents.lines() {
        
        // concatenate frist and last numbers to create a 2 digit number
        let first = get_first_number(line);
        let last = get_last_number(line);
        
        let num = format!("{}{}", first, last).parse::<i32>().unwrap();
        
        sum += num;
        
        println!("{:?} :: [{:?}, {:?}]", line, first, last);
        println!();
    }

    let elapsed = start.elapsed();
    println!("{:?} :: {:?}", sum, elapsed);
}
