use rand::Rng;
use rand::rngs::StdRng;

use crate::generator::{BoundingBox};
use crate::{Tile};

// Checks if two BoundingBox instances interfere with one another
// Used to test if a space has already been taken by another room
fn has_overlap(b0: &BoundingBox, b1: &BoundingBox) -> bool {
    !(
        b0.x >= b1.x_maxima() ||
        b1.x >= b0.x_maxima() ||
        b0.y >= b1.y_maxima() ||
        b1.y >= b0.y_maxima()
    )
}

// Similar to the above function
// Checks if the two BoundingBox instances share a side
fn has_proximity(b0: &BoundingBox, b1: &BoundingBox) -> bool {
    !has_overlap(b0, b1) &&
        (
            b0.y == b1.y_maxima() ||
            b1.y == b0.y_maxima() ||
            b0.x == b1.x_maxima() ||
            b1.x == b0.x_maxima()
        )
}

// Checks if the given room takes up an unallocated space
fn has_proximity_to_any(rooms: &Vec<BoundingBox>, bounds: &BoundingBox) -> bool {
    // immediately return false if no other rooms have been created
    if rooms.len() == 0 { return false; }

    // test against each other room
    for room in rooms.iter() {
        if has_proximity(room, bounds) { return true; }
    }

    // otherwise return false
    false
}

// Checks if a given side already has a door tile
fn has_doors(side: &Vec<&Tile>) -> bool {
    for tile in side.iter() {
        if tile.id == 3 { return true; }
    }

    false
}

// Returns a float that represents the percent of empty tiles in a BoundingBox
fn get_fill_percentage(w: &Vec<Vec<Tile>>, bounds: &BoundingBox) -> f32 {
    let mut count = 0;
    for y in bounds.y_range() {
        for x in bounds.x_range() {
            if w[y][x].id == 0 { count += 1; }
        }
    }

    // calculate the percentage then return
    count as f32 / (bounds.width * bounds.height) as f32
}

// Tries to generate a new room
// Returns an Option enum, if the generation fails it returns None
fn get_new_room(w: &Vec<Vec<Tile>>, prng: &mut StdRng, rooms: &Vec<BoundingBox>, touches: bool) -> Option<BoundingBox> {
    // mult determines the size of the room
    // represents a percent of the total size of the map
    let mult = 0.02f32;

    // generate random height and width of the room
    let height = prng.gen_range(
        (mult.sqrt() * 3f32 / 4f32 * w.len() as f32)..
            (mult.sqrt() * 2f32 * w.len() as f32)) as usize;
    let width = (mult * (w.len() * w[0].len()) as f32) as usize / height;

    // create a BoundingBox for the room
    let mut room = BoundingBox {
        x: 0,
        y: 0,
        height,
        width
    };

    // the first number represents the number of resets before the generation fails
    // the second is the number of locations to test
    let mut attempts = [16, 16];
    return loop {
        // choose a random location
        room.x = prng.gen_range(1..(w[0].len() - room.width - 1));
        room.y = prng.gen_range(1..(w.len() - room.height - 1));

        // test if the location fulfills the requirements for placement
        let percent = get_fill_percentage(w, &room);
        if (percent < 0.20) && ((touches && has_proximity_to_any(rooms, &room)) || (!touches && percent > 0f32)) {
            break Some(room);
        }

        // decrement the attempt counter
        attempts[1] -= 1;

        // reduce the dimensions of the rooms if the placement attempts have failed
        if attempts[1] == 0 {
            let mut finished = false;
            match prng.gen_bool(0.5) {
                true => {
                    room.width -= if room.width > 3 { 1 } else { finished = true; 0 };
                },
                false => {
                    room.height -= if room.height > 3 { 1 } else { finished = true; 0 };
                }
            }

            if finished {
                break None;
            } else {
                // reset placement counter and decrement the reset counter
                attempts[0] -= 1;
                attempts[1] += 32;
            }

            // if there are no more resets remaining, consider the generation a failure
            if attempts[0] == 0 { break None; }
        }
    };
}

// Check if a certain point is contained in any of the generated rooms
fn occupied(rooms: &Vec<BoundingBox>, x: usize, y: usize) -> bool {
    for room in rooms.iter() {
        if room.contains(x, y) { return true; }
    }

    false
}

// Room generation function
// Requires the random number generation instance because room placement should be seeded
pub(crate) fn add_rooms(w: &mut Vec<Vec<Tile>>, prng: &mut StdRng) {
    let mut rooms: Vec<BoundingBox> = Vec::new();

    // attempt to place as many disconnected 'entrance' rooms as possible
    'independent: loop {
        let curr: BoundingBox;
        match get_new_room(w, prng, &rooms, false) {
            Some(room) => { curr = room; },
            None => { break 'independent; }
        }

        // room placement is invalid if another room already takes its place
        for room in rooms.iter() {
            if has_overlap(room, &curr) {
                continue 'independent;
            }
        }

        // append new room to the Vec of rooms
        rooms.push(curr);

    }

    // then, generate connected rooms
    // this step creates the bulk of the dungeon
    'connected: loop {
        let curr: BoundingBox;
        match get_new_room(w, prng, &rooms, true) {
            Some(room) => { curr = room; },
            None => { break 'connected; }
        }

        // generate a new room without appending IF its space is occupied
        for room in rooms.iter() {
            if has_overlap(room, &curr) {
                continue 'connected;
            }
        }

        // append the new room
        rooms.push(curr);

    }

    // fill in the sides of each room
    for room in rooms.iter() {
        for y in room.y_range() {
            for x in room.x_range() {
                if x == room.x || x == room.x_maxima() || y == room.y || y == room.y_maxima() {
                    w[y][x].id = 2;
                } else {
                    w[y][x].id = 0;
                }
            }
        }
    }

    // create doorways and corridors between adjacent rooms
    for room in rooms.iter() {
        let temp = w.clone();
        let sides = room.sides(&temp);

        'side: for side in sides.iter() {
            if has_doors(side) { continue 'side; }

            // generate a list of door/corridor positions for a given side
            let mut possibilities: Vec<Vec<[usize; 2]>> = Vec::new();
            let max_corridor_length = 6;
            'tile: for (n, tile) in side.iter().enumerate() {
                if n == 0 || n == side.len() { continue 'tile; }

                // test for door locations
                if (w[tile.y - 1][tile.x].id == 0 && w[tile.y + 1][tile.x].id == 0) ||
                    (w[tile.y][tile.x - 1].id == 0 && w[tile.y][tile.x + 1].id == 0) {
                    possibilities.push(vec![[tile.y, tile.x]]);
                    continue 'tile;
                }

                // left to right corridor
                if w[tile.y - 1][tile.x].id == 0 && room.contains(tile.x, tile.y - 1) {
                    let mut current: Vec<[usize; 2]> = Vec::new();
                    for y in tile.y..(if tile.y + max_corridor_length >= w.len() { w.len() - 1 } else { tile.y + max_corridor_length }) {
                        if w[y][tile.x].id == 1 || w[y][tile.x].id == 2 || w[y][tile.x].id == 3 {
                            current.push([y, tile.x]);
                        } else if w[y][tile.x].id == 0 && occupied(&rooms, tile.x, y) {
                            possibilities.push(current.clone());
                            continue 'tile;
                        }
                    }
                }

                // right to left corridor
                if w[tile.y + 1][tile.x].id == 0 && room.contains(tile.x, tile.y + 1) {
                    let mut current: Vec<[usize; 2]> = Vec::new();
                    for y in ((if (tile.y as isize - max_corridor_length as isize) < 0isize { 0 } else { tile.y - max_corridor_length })..=tile.y).rev() {
                        if w[y][tile.x].id == 1 || w[y][tile.x].id == 2 || w[y][tile.x].id == 3 {
                            current.push([y, tile.x]);
                        } else if w[y][tile.x].id == 0 && occupied(&rooms, tile.x, y) {
                            possibilities.push(current.clone());
                            continue 'tile;
                        }
                    }
                }

                // top to bottom corridor
                if w[tile.y][tile.x - 1].id == 0 && room.contains(tile.x - 1, tile.y) {
                    let mut current: Vec<[usize; 2]> = Vec::new();
                    for x in tile.x..(if tile.x + max_corridor_length >= w[0].len() { w[0].len() - 1 } else { tile.x + max_corridor_length }) {
                        if w[tile.y][x].id == 1 || w[tile.y][x].id == 2 || w[tile.y][x].id == 3 {
                            current.push([tile.y, x]);
                        } else if w[tile.y][x].id == 0 && occupied(&rooms, x, tile.y){
                            possibilities.push(current.clone());
                            continue 'tile;
                        }
                    }
                }

                // bottom to top corridor
                if w[tile.y][tile.x + 1].id == 0 && room.contains(tile.x + 1, tile.y) {
                    let mut current: Vec<[usize; 2]> = Vec::new();
                    for x in ((if (tile.x as isize - max_corridor_length as isize) < 0isize { 0 } else { tile.x - max_corridor_length })..=tile.x).rev() {
                        if w[tile.y][x].id == 1 || w[tile.y][x].id == 2 || w[tile.y][x].id == 3 {
                            current.push([tile.y, x]);
                        } else if w[tile.y][x].id == 0 && occupied(&rooms, x, tile.y) {
                            possibilities.push(current.clone());
                            continue 'tile;
                        }
                    }
                }
            }

            // choose from a list of possibilities and carve out the corridors/place doors
            if possibilities.len() == 0 { continue 'side; }
            else {
                let index = prng.gen_range(0..possibilities.len());
                for (j, tile) in possibilities[index].iter().enumerate() {
                    w[tile[0]][tile[1]].id = if j == 0 || j == possibilities[index].len() - 1 {
                        3
                    } else {
                        0
                    };
                }

            }
        }
    }
}