use super::super::components::{Coord, Tile, TileMap};
use std::collections::{HashMap, HashSet};

pub type CoordCollection = HashMap<Tile, HashSet<Coord>>;
pub type CoordPairCollection = HashMap<Tile, HashSet<[Coord; 2]>>;

pub fn tile_map_to_coord_collection(map: &TileMap) -> CoordCollection {
    let mut collection: CoordCollection = HashMap::new();
    for (&coord, &tile) in map.iter() {
        match collection.get_mut(&tile) {
            Some(coords) => {
                coords.insert(coord);
            }
            _ => {
                collection.insert(tile, HashSet::from([coord]));
            }
        }
    }
    collection
}

fn create_coord_pair(coords: &HashSet<Coord>) -> HashSet<[Coord; 2]> {
    let ordered_coords: Vec<Coord> = coords.clone().into_iter().collect();
    let mut pairs: HashSet<[Coord; 2]> = HashSet::new();
    for i in 0..ordered_coords.len() {
        for j in (i + 1)..ordered_coords.len() {
            pairs.insert([ordered_coords[i], ordered_coords[j]]);
        }
    }
    pairs
}

#[test]
fn test_tile_map_to_coord_collection() {
    let map: TileMap = HashMap::from([([0, 0], 0), ([1, 1], 1), ([2, 3], 2), ([1, 0], 1)]);
    let expected: CoordCollection = HashMap::from([
        (0, HashSet::from([[0, 0]])),
        (1, HashSet::from([[1, 1], [1, 0]])),
        (2, HashSet::from([[2, 3]])),
    ]);
    let actual = tile_map_to_coord_collection(&map);

    assert_eq!(actual.get(&0), expected.get(&0));
    assert_eq!(actual.get(&1), expected.get(&1));
    assert_eq!(actual.get(&2), expected.get(&2));
}

#[test]
fn test_create_coord_pair() {
    let coords: HashSet<Coord> = HashSet::from([[0, 0], [0, 1], [1, 0], [1, 1]]);
    assert_eq!(create_coord_pair(&coords).len(), 6);
}
