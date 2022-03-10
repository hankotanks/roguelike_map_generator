use map_generator::build;

use std::collections::HashMap;

fn main() {

    let symbols: HashMap<u8, char> = HashMap::from([
        (0, ' '),
        (1, '#'),
        (2, '%')
    ]);

    let world = build(32, 98, None);

    for y in 0..world.len() {
        for x in 0..world[0].len() {
            print!("{}", symbols[&world[y][x].id])
        }
        println!();
    }
}