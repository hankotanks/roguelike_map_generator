use map_generator::build;
use colored::*;

use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let symbols: HashMap<u8, ColoredString> = HashMap::from([
        (0, " ".white().on_black()),
        (1, "#".black().on_blue()),
        (2, "%".black().on_magenta()),
        (3, "+".magenta().on_black())
    ]);
    let now = Instant::now();
    let world = build(32, 64, None, true);
    let elapsed = now.elapsed();

    for y in 0..world.len() {
        for x in 0..world[0].len() {
            print!("{}", symbols[&world[y][x].id])
        }
        println!();
    }

    println!("{:.2?}", elapsed);
}