# map_generator

A library that generates 2D-interconnected cave systems. Optionally, can be compiled into a Python module. 

I created this project (my first attempt at Rust) as an exercise in array manipulation, procedural generation, and algorithmic thinking as a whole. In the future, `map_generator` will serve as a basis for a [traditional roguelike](https://en.wikipedia.org/wiki/Roguelike) focused on spelunking, evasion, terrain traversal, etc...

## Example

<img src="./examples/example.gif" width="256">

## Functions
`generate(height: int, width: int) ⟶ List(List(int))`

`generate_from_seed(height: int, width: int, seed: int) ⟶ List(List(int))`

## Requirements

Rust and Python requirements are specified in `Cargo.toml` and `requirements.txt` respectively. Cargo should handle Rust dependencies automatically; any missing Python modules can be installed with the following:
```
pip install -r requirements.txt
```

## Usage
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
