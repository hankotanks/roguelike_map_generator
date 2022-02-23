mod generator;

use pyo3::prelude::*;
use pyo3::types::PyList;

fn convert_map(w: &Vec<Vec<generator::Tile>>) -> Vec<Vec<u8>> {
    let mut converted = vec![vec![0u8; w[0].len()]; w.len()];
    for (y, row) in w.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            converted[y][x] = tile.id;
        }
    }

    converted
}

// This function serves 'step'
// Both are for testing purposes and won't be needed
fn convert_map_from_py(w: &PyList) -> PyResult<Vec<Vec<generator::Tile>>> {
    let w: Vec<Vec<u8>> = w.extract()?;
    let mut converted = vec![vec![generator::Tile::new(); w[0].len()]; w.len()];
    for (y, row) in w.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            converted[y][x].set(y, x, *val);
        }
    }

    Ok(converted)
}

// A testing function
#[pyfunction]
fn generate_map_from_seed_with_steps(height: usize, width: usize, seed: u64) -> PyResult<Vec<Vec<Vec<u8>>>> {
    let world_states = generator::generate_with_steps(height, width, Some(seed));
    let mut converted_states: Vec<Vec<Vec<u8>>> = vec![];
    for state in world_states.iter() {
        converted_states.push(convert_map(state));
    }

    Ok(converted_states)

}

#[pyfunction]
fn generate_map(height: usize, width: usize) -> PyResult<Vec<Vec<u8>>> {
    let map = generator::generate(height, width, None);
    let map = convert_map(&map);

    Ok(map)
}

#[pyfunction]
fn generate_map_from_seed(height: usize, width: usize, seed: u64) -> PyResult<Vec<Vec<u8>>> {
    let seed = Some(seed);

    let map = generator::generate(height, width, seed);
    let map = convert_map(&map);

    Ok(map)
}

#[pymodule]
fn map_generator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_map, m)?)?;
    m.add_function(wrap_pyfunction!(generate_map_from_seed, m)?)?;
    m.add_function(wrap_pyfunction!(generate_map_from_seed_with_steps, m)?)?;

    Ok(())
}