mod generator;

use std::collections::HashMap;
use pyo3::prelude::*;

fn convert_map(w: &Vec<Vec<generator::Tile>>) -> Vec<Vec<u8>> {
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
    let map = generator::generate(height, width, None);
    let map = convert_map(&map);

    Ok(map)
}

#[pyfunction]
fn generate_from_seed(height: usize, width: usize, seed: u64) -> PyResult<Vec<Vec<u8>>> {
    let seed = Some(seed);

    let map = generator::generate(height, width, seed);
    let map = convert_map(&map);

    Ok(map)
}

// A testing function
#[pyfunction]
fn generate_from_seed_with_steps(height: usize, width: usize, seed: u64) -> PyResult<HashMap<String, Vec<Vec<u8>>>> {
    let world_states = generator::generate_with_steps(height, width, Some(seed));
    let mut converted_states: HashMap<String, Vec<Vec<u8>>> = HashMap::new();
    for (description, state) in &world_states {
        converted_states.insert(description.clone(), convert_map(state));
    }

    Ok(converted_states)
}

#[pymodule]
fn map_generator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate, m)?)?;
    m.add_function(wrap_pyfunction!(generate_from_seed, m)?)?;
    m.add_function(wrap_pyfunction!(generate_from_seed_with_steps, m)?)?;

    Ok(())
}