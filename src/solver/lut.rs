use super::super::components::{Coord, Tile, TileMap};
use std::collections::{HashMap, HashSet};

pub type CoordCollection = HashMap<Tile, Vec<Coord>>;
pub type CoordPairCollection = HashMap<Tile, Vec<[Coord; 2]>>;

pub fn tile_map_to_coord_collection(map: &TileMap) -> CoordCollection {
    let mut collection: CoordCollection = HashMap::new();
    for (&coord, &tile) in map.iter() {
        match collection.get_mut(&tile) {
            Some(coords) => {
                coords.push(coord);
            }
            _ => {
                collection.insert(tile, vec![coord]);
            }
        }
    }
    collection
}

fn create_coord_pair(coords: &Vec<Coord>) -> Vec<[Coord; 2]> {
    let ordered_coords: Vec<Coord> = coords.clone().into_iter().collect();
    let mut pairs: Vec<[Coord; 2]> = Vec::new();
    for i in 0..ordered_coords.len() {
        for j in (i + 1)..ordered_coords.len() {
            pairs.push([ordered_coords[i], ordered_coords[j]]);
        }
    }
    pairs
}

pub fn create_coord_pair_collection(lut: &CoordCollection) -> CoordPairCollection {
    let mut pairs: CoordPairCollection = HashMap::new();
    for (&tile, coords) in lut.iter() {
        pairs.insert(tile, create_coord_pair(coords));
    }
    pairs
}

#[test]
fn test_tile_map_to_coord_collection() {
    let map: TileMap = HashMap::from([([0, 0], 0), ([1, 1], 1), ([2, 3], 2), ([1, 0], 1)]);
    let mut actual = tile_map_to_coord_collection(&map);
    for coords in actual.values_mut() {
        coords.sort();
    }

    let mut actual1: Vec<Coord> = actual[&0].clone();
    let expected1 = vec![[0, 0]];
    assert_eq!(actual1, expected1);

    let mut actual2: Vec<Coord> = actual[&1].clone();
    let expected2 = vec![[1, 0], [1, 1]];
    assert_eq!(actual2, expected2);

    let mut actual3: Vec<Coord> = actual[&2].clone();
    let expected3 = vec![[2, 3]];
    assert_eq!(actual3, expected3);
}

#[test]
fn test_create_coord_pair() {
    let coords: Vec<Coord> = vec![[0, 0], [1, 0], [0, 1], [1, 1]];
    let mut pair_set = create_coord_pair(&coords);
    pair_set.iter_mut().for_each(|coords| coords.sort());
    pair_set.sort();
    assert_eq!(
        pair_set,
        vec![
            [[0, 0], [0, 1]],
            [[0, 0], [1, 0]],
            [[0, 0], [1, 1]],
            [[0, 1], [1, 0]],
            [[0, 1], [1, 1]],
            [[1, 0], [1, 1]]
        ]
    );
}

#[test]
fn test_create_coord_pair_collection() {
    let lut: CoordCollection = HashMap::from([
        (0, vec![[0, 0], [0, 1]]),
        (1, vec![[1, 0], [1, 2], [1, 3], [1, 1]]),
        (2, vec![[2, 0], [2, 1]]),
    ]);
    let mut actual = create_coord_pair_collection(&lut);
    for coords in actual.values_mut() {
        coords.iter_mut().for_each(|pair| pair.sort());
        coords.sort();
    }

    assert_eq!(actual[&0], vec![[[0, 0], [0, 1]]]);
    assert_eq!(
        actual[&1],
        vec![
            [[1, 0], [1, 1]],
            [[1, 0], [1, 2]],
            [[1, 0], [1, 3]],
            [[1, 1], [1, 2]],
            [[1, 1], [1, 3]],
            [[1, 2], [1, 3]]
        ]
    );
    assert_eq!(actual[&2], vec![[[2, 0], [2, 1]]]);
}
