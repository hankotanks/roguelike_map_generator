# map_generator

A library that generates 2D-interconnected cave systems. Optionally, can be compiled into a Python module. 

I created this project (my first shot at Rust) as an exercise in array manipulation, procedural generation, and algorithmic thinking as a whole. In the future, `map_generator` will serve as a basis for a [traditional roguelike](https://en.wikipedia.org/wiki/Roguelike) focused on spelunking, evasion, terrain traversal, etc...

## Example

### Cave generation process

![](/examples/example.gif)

### Dungeons

| ![](/examples/01.PNG) | ![](/examples/02.PNG) |
|:---------------------:|:---------------------:|
| ![](/examples/03.PNG) | ![](/examples/04.PNG) |


## Functions

### Python
Creates a simple cave world. Connectivity between caves is ensured.

`generate(height: int, width: int) ⟶ List(List(int))`

`generate_from_seed(height: int, width: int, seed: int) ⟶ List(List(int))`

Generates a cave with a series of connected rooms and corridors.

`dungeon(height: int, width: int) ⟶ List(List(int))`

`dungeon_from_seed(height: int, width: int, seed: int) ⟶ List(List(int))`

### Rust

`build(height: usize, width: usize, seed: Option<u64>, rooms: bool) -> Vec<Vec<Tile>>`

A random seed will be used if seed is `Option::None` otherwise, the provided `Some` value will be unwrapped and used to seed the generation process.

## Requirements

Rust and Python requirements are specified in `Cargo.toml` and `requirements.txt` respectively. Cargo should handle Rust dependencies automatically; any missing Python modules can be installed with the following:
```
pip install -r requirements.txt
```

## Usage

### Python

```
import os, sys

from map_generator import generate_from_seed

seed = int.from_bytes(os.urandom(8), 'big')
world = generate_from_seed(32, 64, seed)
for row in world:
    for cell in row:
        sys.stdout.write("#" if cell else " ")
    sys.stdout.write("\n")
```

### Rust

```
use map_generator::build;
use std::collections::HashMap;

fn main() {
    let symbols: HashMap<u8, ColoredString> = HashMap::from([
        (0, " "),
        (1, "#"),
        (2, "%"),
        (3, "+")
    ]);
    
    let world = build(32, 64, None, true);
    for y in 0..world.len() {
        for x in 0..world[0].len() {
            print!("{}", symbols[&world[y][x].id])
        }
        println!();
    }
}
```
