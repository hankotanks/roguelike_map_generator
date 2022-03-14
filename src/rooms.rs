use rand::Rng;
use rand::rngs::StdRng;

use crate::generator::BoundingBox;
use crate::Tile;

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
    let height = w.len();
    let width = w[0].len();

    let multiplier = (height as f32 * 0.1414) as usize;
    let dimensions = ((height * width) as f32 * 0.02) as usize;

    let height_range = (multiplier * 3 / 4)..(multiplier * 2);
    let height = prng.gen_range(height_range);
    let width = dimensions / height;

    let mut room = BoundingBox {
        x: 0,
        y: 0,
        height,
        width
    };

    let mut attempts = 64;
    loop {
        room.x = prng.gen_range(1..(w[0].len() - room.width - 1));
        room.y = prng.gen_range(1..(w.len() - room.height - 1));

        let percent = get_percent_empty_in_bounds(w, &room);
        if percent < 0.20 {
            if (touches && touches_any(all_rooms, &room)) || (!touches && percent > 0f32) {
                break;
            }
        } else { attempts -= 1; }

        if attempts == 0 { break; }
    }

    if attempts == 0 {
        return None;
    }

    Some(room)

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

pub(crate) fn construct_rooms(w: &mut Vec<Vec<Tile>>, prng: &mut StdRng) {
    let mut all_rooms: Vec<BoundingBox> = Vec::new();

    //all_rooms.push(get_new_room(w, prng, &all_rooms, false).unwrap());

    'outer: loop {
        let curr: BoundingBox;
        let should_touch = if all_rooms.len() >= 4 { true } else { false };


        match get_new_room(w, prng, &all_rooms, should_touch) {
            Some(room) => { curr = room; },
            None => { break 'outer; }
        }

        for room in all_rooms.iter() {
            if do_rooms_overlap(room, &curr) {
                continue 'outer;
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

            let mut possibilities: Vec<[[usize; 2]; 2]> = Vec::new();
            for tile in side.iter() {
                if (w[tile.y - 1][tile.x].id == 0 && w[tile.y + 1][tile.x].id == 0) ||
                    (w[tile.y][tile.x - 1].id == 0 && w[tile.y][tile.x + 1].id == 0) {
                    possibilities.push([[tile.y, tile.x], [0, 0]]);
                } else if w[tile.y - 1][tile.x].id == 2 && w[tile.y - 2][tile.x].id == 0 && w[tile.y + 1][tile.x].id == 0 {
                    possibilities.push([[tile.y, tile.x], [tile.y - 1, tile.x]])
                } else if w[tile.y - 1][tile.x].id == 0 && w[tile.y + 1][tile.x].id == 2 && w[tile.y + 2][tile.x].id == 0 {
                    possibilities.push([[tile.y, tile.x], [tile.y + 1, tile.x]])
                } else if w[tile.y][tile.x - 1].id == 2 && w[tile.y][tile.x - 2].id == 0 && w[tile.y][tile.x + 1].id == 0 {
                    possibilities.push([[tile.y, tile.x], [tile.y, tile.x - 1]])
                } else if w[tile.y][tile.x - 1].id == 0 && w[tile.y][tile.x + 1].id == 2 && w[tile.y][tile.x + 2].id == 0 {
                    possibilities.push([[tile.y, tile.x], [tile.y, tile.x + 1]])
                }
            }

            if possibilities.len() == 0 { continue 'side; }
            else {
                let index = prng.gen_range(0..possibilities.len());
                w[possibilities[index][0][0]][possibilities[index][0][1]].id = 3;
                if possibilities[index][1][0] != 0 && possibilities[index][1][1] != 0 {
                    w[possibilities[index][1][0]][possibilities[index][1][1]].id = 3;
                }

            }
        }
    }
}