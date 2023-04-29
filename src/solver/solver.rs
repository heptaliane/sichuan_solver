use std::collections::HashMap;

use super::super::components::{Coord, Nodes, Tile, TileMap};
use super::connect::try_get_node_connection;
use super::lut::{create_coord_pair_collection, tile_map_to_coord_collection, CoordCollection};
use super::utils::{get_node_edges, get_size_from_map, pad_tilemap};

fn remove_tiles(map: &TileMap, nodes: &Vec<Nodes>) -> TileMap {
    let mut new_map = map.clone();
    nodes
        .iter()
        .map(|nodes| get_node_edges(&nodes))
        .flatten()
        .for_each(|coord| {
            new_map.remove(&coord);
        });
    new_map
}

fn get_single_pair_tiles(lut: &CoordCollection) -> Vec<[Coord; 2]> {
    let mut pairs: Vec<[Coord; 2]> = Vec::new();

    for collections in lut.values() {
        if collections.len() == 2 {
            let mut coords: Vec<Coord> = collections.clone().into_iter().collect();
            coords.sort();
            pairs.push([coords[0], coords[1]]);
        }
    }

    pairs
}

fn get_trivial_connections(map: &TileMap, map_size: &[usize; 2]) -> Vec<Nodes> {
    let lut = tile_map_to_coord_collection(map);
    let mut nodes: Vec<Nodes> = get_single_pair_tiles(&lut)
        .iter()
        .filter_map(|[coord1, coord2]| try_get_node_connection(coord1, coord2, map, &map_size))
        .collect();
    nodes.sort();
    nodes
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

#[derive(Clone)]
struct SichuanSolverSnapshot {
    map: TileMap,
    map_size: [usize; 2],
    connections: Vec<Nodes>,
    cursor: usize,
    resolved: Vec<Nodes>,
}

impl SichuanSolverSnapshot {
    fn try_new(map: &TileMap, map_size: &[usize; 2]) -> Option<Self> {
        let connections = get_ordered_available_connections(map, map_size);
        match connections.len() {
            0 => None,
            _ => Some(Self {
                map: map.clone(),
                map_size: map_size.clone(),
                connections,
                cursor: 0,
                resolved: Vec::new(),
            }),
        }
    }

    fn resolve(&mut self) {
        let assumed = &self.connections[self.cursor];
        let mut map = remove_tiles(&self.map, &vec![assumed.to_vec()]);

        self.resolved.clear();
        loop {
            let resolved = get_trivial_connections(&map, &self.map_size);
            if resolved.len() == 0 {
                break;
            }

            map = remove_tiles(&map, &resolved);
            self.resolved.extend(resolved);
        }
    }

    fn next(&mut self) -> Result<(), ()> {
        match self.cursor + 1 < self.connections.len() {
            true => {
                self.cursor += 1;
                Ok(())
            }
            false => Err(()),
        }
    }

    fn nodes(&self) -> Vec<Nodes> {
        let mut nodes = vec![self.connections[self.cursor].clone()];
        nodes.extend(self.resolved.clone());
        nodes
    }
}

pub struct SichuanSolver {
    snapshots: Vec<SichuanSolverSnapshot>,
    first_resolved: Vec<Nodes>,
}

impl SichuanSolver {
    pub fn new(map: &TileMap) -> Self {
        let pad_map = pad_tilemap(map);
        let map_size = get_size_from_map(&pad_map);
        let resolved = get_trivial_connections(&pad_map, &map_size);
        match SichuanSolverSnapshot::try_new(&remove_tiles(&pad_map, &resolved), &map_size) {
            Some(mut snapshot) => {
                snapshot.resolve();
                Self {
                    snapshots: vec![snapshot],
                    first_resolved: resolved,
                }
            }
            _ => Self {
                snapshots: Vec::new(),
                first_resolved: resolved,
            },
        }
    }

    fn add_snapshot(&mut self) -> Result<(), ()> {
        match self.snapshots.last() {
            Some(latest_snapshot) => {
                let map = remove_tiles(&latest_snapshot.map, &latest_snapshot.nodes());
                match SichuanSolverSnapshot::try_new(&map, &latest_snapshot.map_size) {
                    Some(mut snapshot) => {
                        snapshot.resolve();
                        self.snapshots.push(snapshot);
                        Ok(())
                    }
                    _ => Err(()),
                }
            }
            _ => Err(()),
        }
    }

    fn update_snapshot(&mut self) -> Result<(), ()> {
        loop {
            match self.snapshots.last_mut() {
                Some(latest_snapshot) => match latest_snapshot.next() {
                    Ok(()) => {
                        latest_snapshot.resolve();
                        return Ok(());
                    }
                    Err(()) => {
                        self.snapshots.pop();
                    }
                },
                _ => return Err(()),
            }
        }
    }

    fn is_completed(&self) -> bool {
        let latest_snapshot = self.snapshots.last().unwrap();
        let map = remove_tiles(&latest_snapshot.map, &latest_snapshot.nodes());
        map.len() == 0
    }

    pub fn solve(&mut self) -> Result<(), ()> {
        loop {
            if self.add_snapshot().is_err() {
                match self.is_completed() {
                    true => return Ok(()),
                    false => {
                        if self.update_snapshot().is_err() {
                            return Err(());
                        }
                    }
                }
            }
        }
    }

    pub fn result(&self) -> Vec<Nodes> {
        let mut nodes = self.first_resolved.clone();
        nodes.extend(
            self.snapshots
                .iter()
                .map(|snapshot| snapshot.nodes())
                .flatten()
                .collect::<Vec<Nodes>>(),
        );
        nodes
    }
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
    assert_eq!(
        remove_tiles(&map, &vec![vec![[0, 0], [0, 1], [1, 1]]]),
        expected1
    );

    let expected2: TileMap = HashMap::from([([0, 0], 0), ([1, 1], 0), ([1, 2], 2), ([2, 2], 2)]);
    assert_eq!(remove_tiles(&map, &vec![vec![[0, 1], [2, 1]]]), expected2);

    let expected3: TileMap = HashMap::from([([0, 0], 0), ([0, 1], 1), ([1, 1], 0), ([2, 1], 1)]);
    assert_eq!(remove_tiles(&map, &vec![vec![[1, 2], [2, 2]]]), expected3);
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
    use std::collections::HashMap;
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

#[test]
fn test_snapshot() {
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

    let map = remove_tiles(&map, &get_trivial_connections(&map, &map_size));
    let mut snapshot = SichuanSolverSnapshot::try_new(&map, &map_size).unwrap();
    assert_eq!(
        snapshot.connections,
        vec![
            [[1, 1], [1, 2]],
            [[1, 1], [2, 1]],
            [[1, 2], [2, 2]],
            [[2, 1], [2, 2]],
            [[0, 0], [0, 2]],
            [[2, 0], [3, 0]],
        ]
    );
    snapshot.resolve();

    assert_eq!(
        snapshot.nodes(),
        vec![
            vec![[1, 1], [1, 2]],
            vec![[0, 3], [1, 3], [1, 0]],
            vec![[2, 1], [2, 2]],
        ]
    );

    assert_eq!(snapshot.next(), Ok(()));
    snapshot.resolve();
    assert_eq!(
        snapshot.nodes(),
        vec![
            vec![[1, 1], [2, 1]],
            vec![[1, 2], [2, 2]],
            vec![[0, 3], [1, 3], [1, 0]],
        ]
    );

    assert_eq!(snapshot.next(), Ok(()));
    snapshot.resolve();
    assert_eq!(
        snapshot.nodes(),
        vec![
            vec![[1, 2], [2, 2]],
            vec![[1, 1], [2, 1]],
            vec![[0, 3], [1, 3], [1, 0]],
        ]
    );

    assert_eq!(snapshot.next(), Ok(()));
    snapshot.resolve();
    assert_eq!(
        snapshot.nodes(),
        vec![
            vec![[2, 1], [2, 2]],
            vec![[1, 1], [1, 2]],
            vec![[0, 3], [1, 3], [1, 0]],
        ]
    );

    assert_eq!(snapshot.next(), Ok(()));
    snapshot.resolve();
    assert_eq!(
        snapshot.nodes(),
        vec![
            vec![[0, 0], [0, 2]],
            vec![[0, 3], [0, 0], [1, 0]],
            vec![[2, 0], [3, 0]],
        ]
    );

    assert_eq!(snapshot.next(), Ok(()));
    snapshot.resolve();
    assert_eq!(
        snapshot.nodes(),
        vec![
            vec![[2, 0], [3, 0]],
            vec![[0, 0], [0, 2]],
            vec![[0, 3], [3, 3], [3, 0], [1, 0]],
        ]
    );

    assert_eq!(snapshot.next(), Err(()));
}

#[test]
fn test_sichuan_solver() {
    /*
     * 0 x 0 1
     * 1 2 2 x
     * 0 2 2 3
     * 0 x x 3
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
    let mut solver = SichuanSolver::new(&map);

    assert_eq!(solver.solve(), Ok(()));

    assert_eq!(
        solver.result(),
        vec![
            vec![[3, 4], [4, 4]],
            vec![[1, 1], [1, 3]],
            vec![[1, 4], [1, 1], [2, 1]],
            vec![[3, 1], [4, 1]],
            vec![[2, 2], [2, 3]],
            vec![[3, 2], [3, 3]],
        ]
    );
}
