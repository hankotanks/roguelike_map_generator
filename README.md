# map_generator

A Python module, written in Rust, that generates 2D, interconnected cave systems.

## Example

<img src="./examples/example.gif" width="256">

## Generation steps

- `random noise` The map grid is randomly seeded with empty/wall tiles in an even ratio. The outer edges are always wall tiles.
- `cellular automata` Each tile in the grid is filled if it has more than 4 adjacent wall tiles. Otherwise it becomes empty. This step is applied 64 times.
- `prune` Rooms that are significantly smaller than the largest room are filled in.
- `connect remaining rooms` Paths are drawn between the center tiles of each room. Tiles along each path are set to empty.
- `polish` Also uses cellular automata, except this step opens up narrow passageways. Each wall tile becomes empty if it has >2 empty neighbors.

## Functions

### `generate(height, width)`

Generates a map with a random seed.

```
Parameters:
    height (int): Number of rows in the generated map
    width  (int): Number of columns
     
Returns:
    list(list(int)): The generated map, 
                     each nested list represents a row
    
map_generator.generate(10, 10)
```

### `generate_from_seed(height, width, seed)`

Generates a map from a provided seed. Functions the same as `generate` with an extra parameter.

```
Additional Parameters:
    seed (int): The initial random noise will be seeded with this number
    
from os import urandom
seed = urandom(16)
map_generator.generate_from_seed(10, 10, seed)
```