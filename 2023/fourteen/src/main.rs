use std::collections::HashMap;
use std::{fs, time::Instant};

fn assess_weight(matrix: &Vec<Vec<char>>) -> usize {
    let mut weight = 0;
    let matrix_len = matrix.len();
    
    for (row, row_data) in matrix.iter().enumerate() {
        for &cell in row_data.iter() {
            if cell == 'O' || cell == 'X' {
                weight += matrix_len - row;
            }
        }
    }
    
    weight
}

fn cycle(matrix: &mut Vec<Vec<char>>, n: i32) -> (Vec<usize>, i32 ){
    // keep track of all the weights and indexes
    let mut weights: Vec<usize> = Vec::new();
    let mut wgt_map: HashMap<usize, i32> = HashMap::new();
    let mut largest_range: [i32; 2] = [0, 0];
    let mut largest_count = 0;

    // add in the initial weight
    weights.push(assess_weight(matrix));

    for i in 0..n {        
        tilt(matrix);
        rotate(matrix);
        tilt(matrix);
        rotate(matrix);
        tilt(matrix);
        rotate(matrix);
        tilt(matrix);
        rotate(matrix);

        
        let weight = assess_weight(matrix);
        
        // print the matrix
        println!();
        println!("{} :: {}", i, weight);
        for row in matrix.iter() {
            for cell in row.iter() {
                print!("{}", cell);
            }
            println!();
        };
        
        if wgt_map.contains_key(&weight) {
            // we have a cycle!
            let first_idx = wgt_map.get(&weight).unwrap();
            let range = [*first_idx, i];
            
            if range[1] - range[0] > largest_range[1] - largest_range[0] {
                largest_range = range;
                largest_count = 1;
            } else if range[1] - range[0] == largest_range[1] - largest_range[0] {
                largest_count += 1;

                if largest_count == 3 {
                    // break
                }
            }
        }
        
        wgt_map.insert(
            weight,
            i,
        );
        
        weights.push(weight);
    }
    
    // store weights in a vector
    let mut wgt_range: Vec<usize> = Vec::new();
    for i in largest_range[0]..largest_range[1] {
        wgt_range.push(weights[i as usize]);
    }

   ( wgt_range,
    largest_range[0])
}

fn rotate(matrix: &mut Vec<Vec<char>>) {
    let n = matrix.len();
    
    for i in 0..n {
        for j in i+1..n {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }

    for i in 0..n {
        matrix[i].reverse();
    }
}

fn tilt(matrix: &mut Vec<Vec<char>>) {
    let n = matrix.len();

    for i in 0..n {
        let mut available_space = 0;

        for j in 0..n {
            let char = matrix[j][i];

            if char == '.' {
                available_space += 1;
                continue;
            }

            if char == '#' {
                available_space = 0;
                continue;
            }

            if available_space > 0 {
                matrix[j][i] = '.';
                matrix[j - available_space][i] = char;
            }
        }
    }
}

fn main() {
    // Load input file 
    let input_path = "/Users/adomaitisc/Development/advent-of-code/2023/fourteen/files/input";

    let contents = fs::read_to_string(input_path)
        .expect("could not open file");

    // log the file name
    println!("{}", input_path);

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        let mut row: Vec<char> = Vec::new();

        for c in line.chars() {
            row.push(c);
        }

        matrix.push(row);
    }
    
    // cycles for our matrix
    let pred_cycles: usize = 1000000000;
    let exec_cycles: i32 = 301;

    // time the cycle function
    let start = Instant::now();

    let (pattern, idx) = cycle(&mut matrix, exec_cycles);
    let duration = start.elapsed();

    // predict the pred_cycles using the pattern
    let pred_idx = (pred_cycles - idx as usize) % pattern.len();
    let pred_wgt = pattern[pred_idx];

    println!();
    println!("{} :: {:?}", pred_wgt, duration);
}

