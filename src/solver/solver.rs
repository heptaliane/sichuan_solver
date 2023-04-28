use std::cmp::Ordering;
use std::collections::HashMap;

use super::super::components::{Coord, Nodes, Tile, TileMap};
use super::connect::try_get_node_connection;
use super::lut::{create_coord_pair_collection, tile_map_to_coord_collection, CoordCollection};

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

fn get_ordered_available_connections(map: &TileMap, map_size: &[usize; 2]) -> Vec<Nodes> {
    /*
     * Connection order:
     * 1. The fewer remaining tiles comes earlier
     * 2. Tile which has more possible connections comes earlier
     * 3. The fewer tile values comes earlier
     */
    let lut = tile_map_to_coord_collection(map);
    let mut pair_collection = create_coord_pair_collection(&lut);
    for pairs in pair_collection.values_mut() {
        pairs.iter_mut().for_each(|pair| pair.sort());
        pairs.sort();
    }

    let nodes: HashMap<Tile, Vec<Nodes>> = pair_collection
        .iter()
        .map(|(&tile, pairs)| {
            (
                tile,
                pairs
                    .iter()
                    .filter_map(|[c1, c2]| try_get_node_connection(c1, c2, map, map_size))
                    .collect::<Vec<Nodes>>(),
            )
        })
        .collect();

    let mut tiles: Vec<Tile> = nodes.iter().map(|(k, _)| k.clone()).collect();
    tiles.sort_unstable_by(|a, b| {
        let (n_coords_a, n_coords_b) = (lut[&a].len(), lut[&b].len());
        if n_coords_a != n_coords_b {
            return n_coords_a.partial_cmp(&n_coords_b).unwrap();
        }

        let (n_nodes_a, n_nodes_b) = (nodes[&a].len(), nodes[&b].len());
        if n_nodes_a != n_nodes_b {
            return n_nodes_b.partial_cmp(&n_nodes_a).unwrap();
        }

        a.partial_cmp(&b).unwrap()
    });
    tiles
        .iter()
        .map(|tile| nodes[&tile].clone())
        .flatten()
        .collect()
}

pub struct SichuanSolver {
    maps: Vec<TileMap>,
    indices: Vec<usize>,
    assumed: Vec<Nodes>,
    resolved: Vec<Nodes>,
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

#[test]
fn test_get_ordered_available_connections() {
    /*
     * 0 x 0 1
     * 1 2 2 x
     * 0 2 2 3
     * 0 x x 3
     *
     * 0: 4 tiles, 2 connections
     * 1: 2 tiles, 0 connections
     * 2: 4 tiles, 4 connections
     * 3: 2 tiles, 1 connections
     */
    let map: TileMap = HashMap::from([
        ([0, 0], 0),
        ([0, 2], 0),
        ([0, 3], 1),
        ([1, 0], 1),
        ([1, 1], 2),
        ([1, 2], 2),
        ([2, 0], 0),
        ([2, 1], 2),
        ([2, 2], 2),
        ([2, 3], 3),
        ([3, 0], 0),
        ([3, 3], 3),
    ]);
    let map_size = [4, 4];

    let actual = get_ordered_available_connections(&map, &map_size);
    let expected = vec![
        [[2, 3], [3, 3]],
        [[1, 1], [1, 2]],
        [[1, 1], [2, 1]],
        [[1, 2], [2, 2]],
        [[2, 1], [2, 2]],
        [[0, 0], [0, 2]],
        [[2, 0], [3, 0]],
    ];
    assert_eq!(actual, expected);
}
