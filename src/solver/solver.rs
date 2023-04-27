use super::super::components::{Coord, Nodes, TileMap};
use super::connect::try_get_node_connection;
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

fn get_node_edges(node: &Nodes) -> [Coord; 2] {
    [node.first().unwrap().clone(), node.last().unwrap().clone()]
}

fn get_trivial_connections(map: &TileMap, map_size: &[usize; 2]) -> Vec<Nodes> {
    let lut = tile_map_to_coord_collection(map);
    get_single_pair_tiles(&lut)
        .iter()
        .filter_map(|[coord1, coord2]| try_get_node_connection(coord1, coord2, map, &map_size))
        .collect()
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

#[test]
fn test_get_trivial_connections() {
    use std::collections::{HashMap, HashSet};
    let reverse = |nodes: &Nodes| {
        let mut reversed = nodes.clone();
        reversed.reverse();
        reversed
    };

    /*
     * 0 x 0 1
     * 1 2 x 1
     * 2 x 3 2
     * 3 x 2 1
     *
     */
    let map: TileMap = HashMap::from([
        ([0, 0], 0),
        ([0, 2], 0),
        ([0, 3], 1),
        ([1, 0], 1),
        ([1, 1], 2),
        ([1, 3], 1),
        ([2, 0], 2),
        ([2, 2], 3),
        ([2, 3], 2),
        ([3, 0], 3),
        ([3, 2], 2),
        ([3, 3], 1),
    ]);
    let map_size = [4, 4];
    let conns = get_trivial_connections(&map, &map_size);
    let expected0: Nodes = vec![[0, 0], [0, 2]];
    let expected3: Nodes = vec![[2, 2], [2, 1], [3, 1], [3, 0]];

    assert_eq!(conns.len(), 2);

    for conn in conns {
        match conn.first().unwrap() {
            &actual if actual == expected0[0] => assert_eq!(conn, expected0),
            &actual if actual == expected0[1] => assert_eq!(conn, reverse(&expected0)),
            &actual if actual == expected3[0] => assert_eq!(conn, expected3),
            &actual if actual == expected3[3] => assert_eq!(conn, reverse(&expected3)),
            _ => panic!(),
        }
    }
}
