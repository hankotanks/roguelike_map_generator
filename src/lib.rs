mod generator;

use pyo3::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::generator::{connect, get_regions, Map, polish, prune, step, Tile};

pub fn build(height: usize, width: usize, seed: Option<u64>) -> Vec<Vec<Tile>> {
    // create an PRNG from the provided seed if it has a value
    let mut prng = match seed {
        Some(s) => SeedableRng::seed_from_u64(s),
        None => StdRng::from_entropy()
    };

    // initialize with all 0s
    let mut world = Map::new(height, width);
    for r in 0..height {
        for c in 0..width {
            if r == 0 || r == height - 1 || c == 0 || c == width - 1 {
                world[r][c].set(r, c, 1)
            } else {
                let rand: u8 = prng.gen_range(0..=1);

                // sets the id of the current tile
                world[r][c].set(r, c, match rand {
                    0 => 0,
                    1 => 1,
                    j => j
                });
            }
        }
    }

    // create cave structure w/ automata
    for _ in 0..64 { step(&mut world); }

    // get a list of all disconnected regions
    let temp = world.clone();
    let regions = get_regions(&temp);

    // fill in smaller rooms
    // after this point, regions is no longer accurate, so they must be recalculated
    // if they are needed again later
    prune(&mut world, &regions);

    // recalculate regions after prune messed up the former Vec
    let temp = world.clone();
    let regions = get_regions(&temp);

    connect(&mut world, &regions);

    // widens paths and smooths out the cave
    for _ in 0..2 { polish(&mut world); }

    world
}


fn convert_map(w: &Vec<Vec<Tile>>) -> Vec<Vec<u8>> {
    let mut converted = vec![vec![0u8; w[0].len()]; w.len()];
    for (y, row) in w.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            converted[y][x] = tile.id;
        }
    }

    converted
}

#[pyfunction]
fn generate(height: usize, width: usize) -> PyResult<Vec<Vec<u8>>> {
    let map = build(height, width, None);
    let map = convert_map(&map);

    Ok(map)
}

#[pyfunction]
fn generate_from_seed(height: usize, width: usize, seed: u64) -> PyResult<Vec<Vec<u8>>> {
    let seed = Some(seed);

    let map = build(height, width, seed);
    let map = convert_map(&map);

    Ok(map)
}

#[pymodule]
fn map_generator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate, m)?)?;
    m.add_function(wrap_pyfunction!(generate_from_seed, m)?)?;

    Ok(())
}