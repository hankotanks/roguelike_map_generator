use rand::Rng;
use std::cmp::Ordering;
use std::ops::Range;
use std::slice::Iter;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Tile {
    pub(crate) y: usize,
    pub(crate) x: usize,
    pub id: u8,
}

impl Tile {
    fn new() -> Tile { Tile { y: 0, x: 0, id: 255 } }

    // Used directly after initialization, because tiles are usually created with default values
    pub(crate) fn set(&mut self, y_pos: usize, x_pos: usize, id: u8) {
        self.y = y_pos;
        self.x = x_pos;
        self.id = id;
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct Region<'a>(Vec<&'a Tile>);

impl<'a> Region<'a> {
    // abstracted functions that apply to 'tiles'
    // 'tiles' should not be accessed outside of the functions in the next block
    pub(crate) fn new() -> Region<'a> { Region { 0: vec![] } }

    pub(crate) fn len(&self) -> usize { self.0.len() }
    fn iter(&self) -> Iter<'_, &'a Tile> { self.0.iter() }
    fn push(&mut self, tile: &'a Tile) { self.0.push(tile); }
    fn contains(&self, tile: &'a Tile) -> bool { self.0.contains(&tile) }
    pub(crate) fn at(&self, index: usize) -> &Tile { self.0[index] }

    // adds a single tile reference to the region
    pub(crate) fn append(&mut self, tile: &'a Tile) {
        self.push(tile)
    }

    // incorporate another region
    fn combine(&mut self, other: Region<'a>) {
        if other.len() > 0 {
            for tile in other.iter() {
                if self.contains(&tile) { continue; };
                self.push(tile)
            }
        }
    }
}

// Regions have no concept of dimensions, so they aren't always appropriate
// A BoundingBox defines an area solely by its dimensions, not by reference
#[derive(Clone, Debug)]
pub(crate) struct BoundingBox {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) width: usize,
    pub(crate) height: usize
}

impl BoundingBox {
    pub(crate) fn x_maxima(&self) -> usize {
         self.x + self.width
    }

    pub(crate) fn y_maxima(&self) -> usize {
        self.y + self.height
    }

    pub(crate) fn y_range(&self) -> Range<usize> {
        self.y..(self.y_maxima() + 1)
    }

    pub(crate) fn x_range(&self) -> Range<usize> {
        self.x..(self.x_maxima() + 1)
    }
}

// Abstraction for creating a new map array
pub(crate) struct Map;

impl Map {
    pub(crate) fn new(height: usize, width: usize) -> Vec<Vec<Tile>> {
        vec![vec![Tile::new(); width]; height]
    }
}

pub(crate) fn step(w: &mut Vec<Vec<Tile>>) {
    // copy current world state
    let mut old_w = Map::new(w.len(), w[0].len());
    for r in 0..old_w.len() {
        for c in 0..old_w[0].len() {
            old_w[r][c] = w[r][c].clone();
        }
    }

    for r in 1..(w.len() - 1) {
        for c in 1..(w[0].len() - 1) {
            let adj = get_neighbor_count(&old_w, r, c);
            w[r][c].id = if adj > 4 { 1 } else if adj < 4 { 0 } else { w[r][c].id };
        }
    }
}

// this method is similar to step but is used to widen the cave after initial generation
pub(crate) fn polish(w: &mut Vec<Vec<Tile>>) {
    let mut old_w = Map::new(w.len(), w[0].len());
    for r in 0..old_w.len() {
        for c in 0..old_w[0].len() {
            old_w[r][c] = w[r][c].clone();
        }
    }

    // tile becomes empty if it has nearby empty space
    for r in 1..(w.len() - 1) {
        for c in 1..(w[0].len() - 1) {
            let adj = get_neighbor_count(&old_w, r, c);
            w[r][c].id = if adj < 5 { 0 } else if adj > 6 { 1 } else { w[r][c].id };
        }
    }
}

// returns number of neighbors, including diagonal tiles
fn get_neighbor_count(w: &Vec<Vec<Tile>>, r: usize, c: usize) -> usize {
    let mut count = 0;
    for i in (r - 1)..=(r + 1) {
        for j in (c - 1)..=(c + 1) {
            if i == r && j == c { continue; }
            count += if w[i][j].id != 0 { 1 } else { 0 };
        }
    }
    count
}

// Return a list of regions (enclosed caves) using a flood fill algorithm
pub(crate) fn get_regions(w: &Vec<Vec<Tile>>) -> Vec<Region> {
    // the number of tiles tested is dependent on the size of the region
    let tries = w.len() * w[0].len() / 2;

    // will be populated with individual regions
    let mut regions: Vec<Region> = Vec::new();

    for _ in 0..tries {
        let y = rand::thread_rng().gen_range(0..w.len());
        let x = rand::thread_rng().gen_range(0..w[0].len());
        let tile: &Tile = &w[y][x];

        // check if tile has been handled
        let mut included = false;
        for region in regions.iter() {
            if region.contains(tile) { included = true }
        }

        // begin flood fill if tile isn't already part of a region
        if !included && tile.id == 0 {
            regions.push(flood_fill_init(w, tile));
        }
    }

    regions
}

// takes a list of Region instances, returns the one with the most tiles
fn get_largest_region<'a>(regions: &'a Vec<Region>) -> &'a Region<'a> {
    let mut largest = &regions[0];

    for region in regions.iter() {
        if region.len() > largest.len() { largest = region; }
    }

    largest
}

// Set the id of each tile in region to 'with'
fn set_all_tiles_in_region(w: &mut Vec<Vec<Tile>>, region: &Region, with: u8) {
    for tile in region.iter() {
        w[tile.y][tile.x].id = with;
    }
}

// A wrapper for flood_fill() so 'filled' isn't required...
// Not strictly necessary, but it de-clutters the first call
fn flood_fill_init<'a>(w: &'a Vec<Vec<Tile>>, tile: &'a Tile) -> Region<'a> {
    flood_fill(w, &mut Region::new(), tile)
}

// Recursively search for nearby tiles that are empty, return the complete list as a Region
fn flood_fill<'b>(w: &'b Vec<Vec<Tile>>, filled: &mut Region<'b>, tile: &'b Tile) -> Region<'b> {
    let mut region = Region::new();

    // only append tile to the current Region if it is empty
    if tile.id == 0 {
        region.append(tile);
    }

    // the current Region is combined with the Region represented previously tested tiles
    filled.combine(region.clone());

    // y and x must signed since a difference is calculated
    let y = tile.y as isize;
    let x = tile.x as isize;

    // ensure the current sub Region doesn't become deallocated
    let mut sub: Region;
    let offsets: [[isize; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];
    for offset in offsets.iter() {
        // define coordinates w/ offsets for convenience
        let oy = (y + offset[0]) as usize;
        let ox = (x + offset[1]) as usize;

        // test if Tile is empty and if it has NOT been tested previously
        if w[oy][ox].id == 0u8 && !filled.contains(&&w[oy][ox]) {
            sub = flood_fill(w, filled, &w[oy][ox]);
            region.combine(sub);
        }
    }

    region
}

// Returns a bounding box for the given
// 1st tile is minima
// 2nd tile is maxima
// NOTE: extrema may NOT be a member of the region they represent
fn find_extrema(region: &Region) -> BoundingBox {
    let mut minima: Option<[usize; 2]> = None;
    let mut maxima: Option<[usize; 2]> = None;

    for tile in region.iter() {
        // update minima is current tile has a smaller y or x coordinate
        minima = match minima {
            None => Some([tile.y, tile.x]),
            Some([y, x]) =>
                Some([if tile.y < y { tile.y } else { y }, if tile.x < x { tile.x } else { x }])
        };

        // do the same, testing for maxima this time
        maxima = match maxima {
            None => Some([tile.y, tile.x]),
            Some([y, x]) =>
                Some([if tile.y > y { tile.y } else { y }, if tile.x > x { tile.x } else { x }])
        };
    }

    // unwrap coordinates before returning the array of Tile extrema
    // this is safe to do because Option is used to test for initialization
    // rather than any sort of 'null' value
    let minima = minima.unwrap();
    let maxima = maxima.unwrap();

    // construct the BoundingBox and return
    BoundingBox {
        x: minima[1],
        y: minima[0],
        width: maxima[1] - minima[1],
        height: maxima[0] - minima[0]
    }
}

// returns the tile at the center of the region's bounding box
// NOTE: center tile may NOT be a member of the supplied region
fn find_center_tile<'a>(w: &'a Vec<Vec<Tile>>, region: &Region) -> &'a Tile {
    // get bounding tiles
    let bb = find_extrema(region);
    let maxima = w[bb.y][bb.x];
    let minima = w[bb.y_maxima()][bb.x_maxima()];

    // REMINDER: this is integer division, it will round down
    let center_x = (maxima.x + minima.x) / 2;
    let center_y = (maxima.y + minima.y) / 2;

    // return tile
    &w[center_y][center_x]
}

// Returns the distance squared between two tiles
// The square root function is costly, so it's more efficient to compare squares of distance
fn get_distance_sq(t1: &Tile, t2: &Tile) -> usize {
    // must case to isize, because difference could result in a negative overflow
    let dx = t1.x as isize - t2.x as isize;
    let dy = t1.y as isize - t2.y as isize;

    // distance will always be positive, so it is returned as usize
    (dx.pow(2) + dy.pow(2)) as usize
}

// Used IFF the center tile is not actually a member of the region it represents
fn get_tile_closest_to_center<'a>(region: &'a Region<'a>, center: &'a Tile) -> &'a Tile {
    let mut closest: &Tile = region.at(1);
    let mut closest_dist: usize = get_distance_sq(center, closest);
    for tile in region.iter() {
        let dist = get_distance_sq(center, tile);
        if dist < closest_dist {
            closest = tile;
            closest_dist = dist;
        }
    }

    closest
}

// Returns a Region that connects the tiles of the two regions passed to the function
fn find_connection<'a>(w: &'a Vec<Vec<Tile>>, r1: &Region, r2: &Region) -> Region<'a> {
    // find center tiles for each region
    let mut r1_center = find_center_tile(w, r1);
    let mut r2_center = find_center_tile(w, r2);

    // account for regions that don't include their center Tile
    if r1_center.id != 0 {
        r1_center = get_tile_closest_to_center(r1, r1_center);
    }

    if r2_center.id != 0 {
        r2_center = get_tile_closest_to_center(r2, r2_center);
    }

    // calculate the slope of the line connecting the two centers
    let dx = (r1_center.x as isize - r2_center.x as isize) as f32;
    let dy = (r1_center.y as isize - r2_center.y as isize) as f32;
    let slope = dy / dx;

    // ensure that the x coordinates are used in the appropriate order
    let min_x;
    let max_x;
    if r1_center.x < r2_center.x {
        min_x = r1_center.x;
        max_x = r2_center.x;
    } else {
        min_x = r2_center.x;
        max_x = r1_center.x;
    }


    // construct the Region
    let mut pathway = Region::new();
    for x in min_x..=max_x {
        // calculate the y value at the given x
        let line_y = slope * x as f32 - slope * r1_center.x as f32 + r1_center.y as f32;
        let line_y = line_y as usize;

        // check if it falls in bounds
        // add both the calculated tile and the one beneath it to the Region
        // this ensures that the polish() method is able to expand each path
        if line_y < w.len() {
            pathway.append(&w[line_y][x]);
            pathway.append(&w[line_y - 1][x]);
        }
    }

    pathway
}

// Determines if a region should be filled
// Min size represents the largest region * the threshold percent
// It is calculated in prune()
fn should_be_pruned(region: &Region, min_size: usize) -> bool {
    match region.len().cmp(&min_size) {
        Ordering::Less => true,
        Ordering::Equal => false,
        Ordering::Greater => false,
    }
}

// Fills regions below a certain size
// Relative to the largest region
pub(crate) fn prune(w: &mut Vec<Vec<Tile>>, regions: &Vec<Region>) {
    let threshold = 0.2;
    let largest: &Region = get_largest_region(&regions);
    let threshold_size = (largest.len() as f32 * threshold) as usize;

    // fill each region that should be pruned
    for region in regions.iter() {
        if should_be_pruned(region, threshold_size) {
            set_all_tiles_in_region(w, region, 1);
        }
    }
}

// Fills each path found by the find_all_connections() function
pub(crate) fn connect(w: &mut Vec<Vec<Tile>>, regions: &Vec<Region>) {
    let temp = w.clone();
    let largest: &Region = get_largest_region(regions);

    // construct the Vec of individual path regions
    let mut paths: Vec<Region> = Vec::new();
    for region in regions.iter() {
        if region != largest {
            let path = find_connection(&temp, largest, &region);
            paths.push(path);
        }
    }

    // fill every path Region that was constructed earlier
    for path in paths.iter() {
        set_all_tiles_in_region(w, path, 0);
    }
}