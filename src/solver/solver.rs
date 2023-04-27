use super::super::components::{Coord, TileMap};
use super::lut::{tile_map_to_coord_collection, CoordCollection};

fn remove_tiles(map: &TileMap, coords: Vec<Coord>) -> TileMap {
    let mut new_map = map.clone();
    for coord in coords {
        new_map.remove(&coord);
    }
    new_map
}

fn get_single_pair_tiles(lut: &CoordCollection) -> Vec<[Coord; 2]> {
    let mut pairs: Vec<[Coord; 2]> = Vec::new();

    for collections in lut.values() {
        if collections.len() == 2 {
            let coords: Vec<Coord> = collections.clone().into_iter().collect();
            pairs.push([coords[0], coords[1]]);
        }
    }

    pairs
}

#[test]
fn test_remove_tiles() {
    use std::collections::HashMap;
    let map: TileMap = HashMap::from([
        ([0, 0], 0),
        ([0, 1], 1),
        ([1, 1], 0),
        ([1, 2], 2),
        ([2, 1], 1),
        ([2, 2], 2),
    ]);

    let expected1: TileMap = HashMap::from([([0, 1], 1), ([1, 2], 2), ([2, 1], 1), ([2, 2], 2)]);
    assert_eq!(remove_tiles(&map, vec![[0, 0], [1, 1]]), expected1);

    let expected2: TileMap = HashMap::from([([0, 0], 0), ([1, 1], 0), ([1, 2], 2), ([2, 2], 2)]);
    assert_eq!(remove_tiles(&map, vec![[0, 1], [2, 1]]), expected2);

    let expected3: TileMap = HashMap::from([([0, 0], 0), ([0, 1], 1), ([1, 1], 0), ([2, 1], 1)]);
    assert_eq!(remove_tiles(&map, vec![[1, 2], [2, 2]]), expected3);
}

#[test]
fn test_get_single_pair_tiles() {
    use std::collections::{HashMap, HashSet};
    let map: TileMap = HashMap::from([
        ([0, 0], 0),
        ([0, 1], 1),
        ([1, 1], 0),
        ([1, 2], 0),
        ([2, 1], 1),
        ([2, 2], 0),
    ]);
    let lut = tile_map_to_coord_collection(&map);
    let coords: Vec<HashSet<Coord>> = get_single_pair_tiles(&lut)
        .iter()
        .map(|pair| HashSet::from(pair.clone()))
        .collect();
    let expected = vec![HashSet::from([[0, 1], [2, 1]])];

    assert_eq!(coords, expected);
}
