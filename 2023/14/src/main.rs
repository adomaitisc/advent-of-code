
use std::fs;

fn shift_os(col: &mut Vec<char>) -> usize {
    let mod_col = col.clone();
    
    let mut spaces = 0;
    let mut sum = 0;

    for (i, ch) in mod_col.iter().enumerate() {

        if ch == &'.' { spaces += 1; }

        if ch == &'#' { spaces = 0; }

        if ch == &'O' {
            // move this i back spaces
            col[i - spaces] = 'O';
            col[i] = '.';
            sum += mod_col.len() - (i - spaces)
        }   
    }

    sum
}

fn main() {
    // Load input file 
    let input_path = "/Users/adomaitisc/Development/advent-of-code/14/main/input2";

    let contents = fs::read_to_string(input_path)
        .expect("Not not");

    // read first line to find the number of columns
    let mut col_count = 0;

    for c in contents.chars() {
        if c == '\n' {
            break;
        }
        col_count += 1;
    }

    // count the number of rows
    let row_count = contents.lines().count();

    // allocate our array of columns
    let mut all_cols: Vec<Vec<char>> = vec![Vec::with_capacity(row_count); col_count];

    // populate the columns
    let mut col_idx = 0;

    for c in contents.chars() {
        if c == '\n' {
            col_idx = 0; // Reset column index at the end of each row
            continue;
        }
        all_cols[col_idx].push(c);
        col_idx += 1;
    }
    
    let mut weight = 0;

    for col in &mut all_cols {
        weight += shift_os(col);
    }

    println!("weight is: {}", weight);
}
