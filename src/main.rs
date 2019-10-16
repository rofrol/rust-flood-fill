use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    let mut state = [[0u8; 4]; 6];
    let mut visited = [[0u8; 4]; 6];
    state[0][1] = 42;
    println!("{:?}", state);

    let mut colors = HashMap::new();

    for (x, row) in state.iter().enumerate() {
        //println!("{:?}", row);
        for (y, cell) in row.iter().enumerate() {
            println!("{:?}", cell);
            let val = colors.get(cell).unwrap_or(&0);
            colors.insert(cell, val + 1);
        }
    }
    println!("len: {}", state.len());
    println!("0, 0: {}", state[0][0]);
    println!("colors: {:?}", colors);
    println!("colors: {:?}", colors);
}
