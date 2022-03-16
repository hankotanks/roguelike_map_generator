use rand::Rng;
use rand::rngs::StdRng;

use crate::generator::{BoundingBox};
use crate::{Tile};

fn do_rooms_overlap(b0: &BoundingBox, b1: &BoundingBox) -> bool {
    !(
        b0.x >= b1.x_maxima() ||
        b1.x >= b0.x_maxima() ||
        b0.y >= b1.y_maxima() ||
        b1.y >= b0.y_maxima()
    )
}

fn do_rooms_touch(b0: &BoundingBox, b1: &BoundingBox) -> bool {
    !do_rooms_overlap(b0, b1) &&
        (
            b0.y == b1.y_maxima() ||
            b1.y == b0.y_maxima() ||
            b0.x == b1.x_maxima() ||
            b1.x == b0.x_maxima()
        )
}

fn get_percent_empty_in_bounds(w: &Vec<Vec<Tile>>, bounds: &BoundingBox) -> f32 {
    let mut count = 0;
    for y in bounds.y_range() {
        for x in bounds.x_range() {
            if w[y][x].id == 0 { count += 1; }
        }
    }

    count as f32 / (bounds.width * bounds.height) as f32
}

fn touches_any(all_rooms: &Vec<BoundingBox>, r: &BoundingBox) -> bool {
    if all_rooms.len() == 0 { return false; }

    for room in all_rooms.iter() {
        if do_rooms_touch(room, r) { return true; }
    }

    false
}

fn get_new_room(w: &Vec<Vec<Tile>>, prng: &mut StdRng, all_rooms: &Vec<BoundingBox>, touches: bool) -> Option<BoundingBox> {
    let mult = 0.02f32;
    let height = prng.gen_range(
        (mult.sqrt() * 3f32 / 4f32 * w.len() as f32)..
            (mult.sqrt() * 2f32 * w.len() as f32)) as usize;
    let width = (mult * (w.len() * w[0].len()) as f32) as usize / height;

    let mut room = BoundingBox {
        x: 0,
        y: 0,
        height,
        width
    };

    let mut attempts = 32;
    return loop {
        room.x = prng.gen_range(1..(w[0].len() - room.width - 1));
        room.y = prng.gen_range(1..(w.len() - room.height - 1));

        let percent = get_percent_empty_in_bounds(w, &room);

        if (percent < 0.20) && ((touches && touches_any(all_rooms, &room)) || (!touches && percent > 0f32)) {
            break Some(room);
        }

        attempts -= 1;

        if attempts == 0 {
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
                println!("Reset attempts!");
                attempts += 32;
            }
        }
    };
}

fn has_doors(side: &Vec<&Tile>) -> bool {
    for tile in side.iter() {
        if tile.id == 3 { return true; }
    }

    false
}

fn get_sides_of_room<'a>(w: &'a Vec<Vec<Tile>>, bounds: &BoundingBox) -> Vec<Vec<&'a Tile>> {
    let mut sides: Vec<Vec<&Tile>> = Vec::new();

    let mut x_side: Vec<&Tile> = Vec::new();
    let mut x_maxima_side: Vec<&Tile> = Vec::new();
    for y in bounds.y_range() {
        x_side.push(&w[y][bounds.x]);
        x_maxima_side.push(&w[y][bounds.x_maxima()]);
    }

    sides.push(x_side);
    sides.push(x_maxima_side);

    let mut y_side: Vec<&Tile> = Vec::new();
    let mut y_maxima_side: Vec<&Tile> = Vec::new();
    for x in bounds.x_range() {
        y_side.push(&w[bounds.y][x]);
        y_maxima_side.push(&w[bounds.y_maxima()][x]);
    }

    sides.push(y_side);
    sides.push(y_maxima_side);

    sides
}

fn contained_in_rooms(rooms: &Vec<BoundingBox>, x: usize, y: usize) -> bool {
    for room in rooms.iter() {
        if room.contains(x, y) { return true; }
    }

    false
}

pub(crate) fn construct_rooms(w: &mut Vec<Vec<Tile>>, prng: &mut StdRng) {
    let mut all_rooms: Vec<BoundingBox> = Vec::new();

    'independent: loop {
        let curr: BoundingBox;
        match get_new_room(w, prng, &all_rooms, false) {
            Some(room) => { curr = room; },
            None => { break 'independent; }
        }

        for room in all_rooms.iter() {
            if do_rooms_overlap(room, &curr) {
                continue 'independent;
            }
        }

        all_rooms.push(curr);

    }

    'connected: loop {
        let curr: BoundingBox;

        match get_new_room(w, prng, &all_rooms, true) {
            Some(room) => { curr = room; },
            None => { break 'connected; }
        }

        for room in all_rooms.iter() {
            if do_rooms_overlap(room, &curr) {
                continue 'connected;
            }
        }

        all_rooms.push(curr);

    }

    for room in all_rooms.iter() {
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

    for room in all_rooms.iter() {
        let temp = w.clone();
        let sides = get_sides_of_room(&temp, room);

        'side: for side in sides.iter() {
            if has_doors(side) { continue 'side; }

            let mut possibilities: Vec<Vec<[usize; 2]>> = Vec::new();
            let max_corridor_length = 6;
            'tile: for (n, tile) in side.iter().enumerate() {
                if n == 0 || n == side.len() { continue 'tile; }

                if (w[tile.y - 1][tile.x].id == 0 && w[tile.y + 1][tile.x].id == 0) ||
                    (w[tile.y][tile.x - 1].id == 0 && w[tile.y][tile.x + 1].id == 0) {
                    possibilities.push(vec![[tile.y, tile.x]]);
                    continue 'tile;
                }

                if w[tile.y - 1][tile.x].id == 0 && room.contains(tile.x, tile.y - 1) {
                    let mut current: Vec<[usize; 2]> = Vec::new();
                    for y in tile.y..(if tile.y + max_corridor_length >= w.len() { w.len() - 1 } else { tile.y + max_corridor_length }) {
                        if w[y][tile.x].id == 1 || w[y][tile.x].id == 2 || w[y][tile.x].id == 3 {
                            current.push([y, tile.x]);
                        } else if w[y][tile.x].id == 0 && contained_in_rooms(&all_rooms, tile.x, y) {
                            possibilities.push(current.clone());
                            continue 'tile;
                        }
                    }
                }

                if w[tile.y + 1][tile.x].id == 0 && room.contains(tile.x, tile.y + 1) {
                    let mut current: Vec<[usize; 2]> = Vec::new();
                    for y in ((if (tile.y as isize - max_corridor_length as isize) < 0isize { 0 } else { tile.y - max_corridor_length })..=tile.y).rev() {
                        if w[y][tile.x].id == 1 || w[y][tile.x].id == 2 || w[y][tile.x].id == 3 {
                            current.push([y, tile.x]);
                        } else if w[y][tile.x].id == 0 && contained_in_rooms(&all_rooms, tile.x, y) {
                            possibilities.push(current.clone());
                            continue 'tile;
                        }
                    }
                }

                if w[tile.y][tile.x - 1].id == 0 && room.contains(tile.x - 1, tile.y) {
                    let mut current: Vec<[usize; 2]> = Vec::new();
                    for x in tile.x..(if tile.x + max_corridor_length >= w[0].len() { w[0].len() - 1 } else { tile.x + max_corridor_length }) {
                        if w[tile.y][x].id == 1 || w[tile.y][x].id == 2 || w[tile.y][x].id == 3 {
                            current.push([tile.y, x]);
                        } else if w[tile.y][x].id == 0 && contained_in_rooms(&all_rooms, x, tile.y){
                            possibilities.push(current.clone());
                            continue 'tile;
                        }
                    }
                }

                if w[tile.y][tile.x + 1].id == 0 && room.contains(tile.x + 1, tile.y) {
                    let mut current: Vec<[usize; 2]> = Vec::new();
                    for x in ((if (tile.x as isize - max_corridor_length as isize) < 0isize { 0 } else { tile.x - max_corridor_length })..=tile.x).rev() {
                        if w[tile.y][x].id == 1 || w[tile.y][x].id == 2 || w[tile.y][x].id == 3 {
                            current.push([tile.y, x]);
                        } else if w[tile.y][x].id == 0 && contained_in_rooms(&all_rooms, x, tile.y) {
                            possibilities.push(current.clone());
                            continue 'tile;
                        }
                    }
                }

                /*
                else if w[tile.y - 1][tile.x].id == 2 && w[tile.y - 2][tile.x].id == 0 && w[tile.y + 1][tile.x].id == 0 {
                    possibilities.push([[tile.y, tile.x], [tile.y - 1, tile.x]])
                } else if w[tile.y - 1][tile.x].id == 0 && w[tile.y + 1][tile.x].id == 2 && w[tile.y + 2][tile.x].id == 0 {
                    possibilities.push([[tile.y, tile.x], [tile.y + 1, tile.x]])
                } else if w[tile.y][tile.x - 1].id == 2 && w[tile.y][tile.x - 2].id == 0 && w[tile.y][tile.x + 1].id == 0 {
                    possibilities.push([[tile.y, tile.x], [tile.y, tile.x - 1]])
                } else if w[tile.y][tile.x - 1].id == 0 && w[tile.y][tile.x + 1].id == 2 && w[tile.y][tile.x + 2].id == 0 {
                    possibilities.push([[tile.y, tile.x], [tile.y, tile.x + 1]])
                }
                 */
            }

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