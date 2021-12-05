use super::components::{Coord, Nodes, Tile, TileMap};
use super::connect::find_connection;
use std::cmp::PartialEq;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct ConnectionInfo {
    tile: Tile,
    pair: [Coord; 2],
    nodes: Nodes,
}

type TileLut = HashMap<Tile, Vec<Coord>>;
type ConnectionLut = HashMap<Tile, Vec<ConnectionInfo>>;

fn create_tile_lookup_table(map: &TileMap) -> TileLut {
    let shape = map.shape();
    let mut lut: TileLut = HashMap::new();

    for x in 0..shape[0] {
        for y in 0..shape[1] {
            if let Some(tile) = map[[x, y]] {
                match lut.get_mut(&tile) {
                    Some(coords) => coords.push([x, y]),
                    _ => {
                        lut.insert(tile, vec![[x, y]]);
                    }
                };
            }
        }
    }
    lut
}

fn collect_connection_single_pair_tiles(map: &TileMap) -> Vec<ConnectionInfo> {
    let lut = create_tile_lookup_table(map);
    let mut connections: Vec<ConnectionInfo> = Vec::new();

    for (&tile, coords) in lut.iter().filter(|(_, coord)| coord.len() == 2) {
        match find_connection(map, &coords[0], &coords[1]) {
            Some(nodes) => {
                println!("tile: {}", tile);
                connections.push(ConnectionInfo {
                    tile: tile,
                    pair: [coords[0], coords[1]],
                    nodes: nodes,
                });
            }
            None => (),
        }
    }
    connections
}

fn remove_connected_tiles(map: &TileMap, connections: &Vec<ConnectionInfo>) -> TileMap {
    let mut new_map = map.clone();
    for connection in connections {
        for coord in connection.pair {
            new_map[coord] = None;
        }
    }
    new_map
}

fn collect_connection_single_pair_tiles_loop(map: &TileMap) -> Vec<ConnectionInfo> {
    let mut connections = collect_connection_single_pair_tiles(map);
    let mut current_map = remove_connected_tiles(map, &connections);

    loop {
        let conn = collect_connection_single_pair_tiles(&current_map);
        if conn.len() == 0 {
            break;
        }

        current_map = remove_connected_tiles(&current_map, &conn);
        connections.extend(conn);
    }
    connections
}

fn get_pair_indices(n_items: usize) -> Vec<[usize; 2]> {
    let mut indices: Vec<[usize; 2]> = Vec::new();
    for i in 0..(n_items - 1) {
        for j in (i + 1)..n_items {
            indices.push([i, j]);
        }
    }
    indices
}

fn create_tile_connection_lookup_table(map: &TileMap, maplut: &TileLut) -> ConnectionLut {
    let mut lut: ConnectionLut = HashMap::new();

    for (&tile, coords) in maplut.iter() {
        let mut connections: Vec<ConnectionInfo> = Vec::new();
        for [i, j] in get_pair_indices(coords.len()) {
            match find_connection(map, &coords[i], &coords[j]) {
                Some(nodes) => connections.push(ConnectionInfo {
                    tile: tile,
                    pair: [coords[i], coords[j]],
                    nodes: nodes,
                }),
                _ => (),
            }
        }
        lut.insert(tile, connections);
    }
    lut
}

#[test]
fn test_create_tile_lookup_table() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(0), Some(1), Some(2)],
        [Some(3), Some(0), Some(0), Some(1)],
        [Some(2), Some(1), Some(1), Some(3)],
    ]);

    let lut = create_tile_lookup_table(&map);
    assert_eq!(lut.get(&0), Some(&vec![[0, 0], [0, 1], [1, 1], [1, 2]]));
    assert_eq!(lut.get(&1), Some(&vec![[0, 2], [1, 3], [2, 1], [2, 2]]));
    assert_eq!(lut.get(&2), Some(&vec![[0, 3], [2, 0]]));
    assert_eq!(lut.get(&3), Some(&vec![[1, 0], [2, 3]]));
    assert_eq!(lut.get(&4), None);
    assert_eq!(lut.len(), 4);
}
#[test]
fn test_collect_connection_single_pair_tiles() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(0), Some(1), Some(2)],
        [Some(3), Some(1), None, Some(2)],
        [None, None, Some(0), Some(4)],
        [Some(0), Some(3), None, None],
        [Some(4), Some(0), Some(0), None],
    ]);
    /*
     * 0 0 1 2
     * 3 1 - 2
     * - - 0 4
     * 0 3 - -
     * 4 0 0 -
     */
    let mut connections = collect_connection_single_pair_tiles(&map);
    connections.sort_by(|a, b| a.tile.cmp(&b.tile));
    assert_eq!(
        connections[0],
        ConnectionInfo {
            tile: 1,
            pair: [[0, 2], [1, 1]],
            nodes: [Some([0, 2]), Some([1, 2]), Some([1, 1]), None],
        }
    );
    assert_eq!(
        connections[1],
        ConnectionInfo {
            tile: 2,
            pair: [[0, 3], [1, 3]],
            nodes: [Some([0, 3]), Some([1, 3]), None, None],
        }
    );
    assert_eq!(
        connections[2],
        ConnectionInfo {
            tile: 3,
            pair: [[1, 0], [3, 1]],
            nodes: [Some([1, 0]), Some([2, 0]), Some([2, 1]), Some([3, 1])],
        }
    );
    assert_eq!(connections.len(), 3);
}

#[test]
fn test_remove_connected_tiles() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(0), Some(1), Some(2)],
        [Some(3), Some(1), Some(4), Some(2)],
        [Some(4), Some(5), Some(5), Some(0)],
        [Some(0), Some(3), Some(0), Some(0)],
    ]);
    /*
     * 0 0 1 2
     * 3 1 4 2
     * 4 5 5 0
     * 0 3 0 0
     */
    let map2: TileMap = arr2(&[
        [Some(0), Some(0), Some(1), None],
        [Some(3), Some(1), Some(4), None],
        [Some(4), None, None, Some(0)],
        [Some(0), Some(3), Some(0), Some(0)],
    ]);
    /*
     * 0 0 1 -
     * 3 1 4 -
     * 4 - - 0
     * 0 3 0 0
     */
    let connections = collect_connection_single_pair_tiles(&map);
    assert_eq!(remove_connected_tiles(&map, &connections), map2);
}

#[test]
fn test_collect_connection_single_pair_tiles_loop() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(0), Some(1), Some(2)],
        [Some(3), Some(1), Some(4), Some(2)],
        [Some(4), Some(5), Some(5), Some(0)],
        [Some(0), Some(3), Some(0), Some(0)],
    ]);
    /*
     * 0 0 1 2
     * 3 1 4 2
     * 4 5 5 0
     * 0 3 0 0
     */
    let mut connections = collect_connection_single_pair_tiles_loop(&map);
    connections.sort_by(|a, b| a.tile.cmp(&b.tile));
    assert_eq!(connections.len(), 5);
    assert_eq!(
        connections[0],
        ConnectionInfo {
            tile: 1,
            pair: [[0, 2], [1, 1]],
            nodes: [Some([0, 2]), Some([1, 2]), Some([1, 1]), None],
        }
    );
    assert_eq!(
        connections[1],
        ConnectionInfo {
            tile: 2,
            pair: [[0, 3], [1, 3]],
            nodes: [Some([0, 3]), Some([1, 3]), None, None],
        }
    );
    assert_eq!(
        connections[2],
        ConnectionInfo {
            tile: 3,
            pair: [[1, 0], [3, 1]],
            nodes: [Some([1, 0]), Some([2, 0]), Some([2, 1]), Some([3, 1])],
        }
    );
    assert_eq!(
        connections[3],
        ConnectionInfo {
            tile: 4,
            pair: [[1, 2], [2, 0]],
            nodes: [Some([1, 2]), Some([2, 2]), Some([2, 0]), None],
        }
    );
    assert_eq!(
        connections[4],
        ConnectionInfo {
            tile: 5,
            pair: [[2, 1], [2, 2]],
            nodes: [Some([2, 1]), Some([2, 2]), None, None],
        }
    );
}

#[test]
fn test_get_pair_indices() {
    assert_eq!(get_pair_indices(2), [[0, 1]]);
    assert_eq!(
        get_pair_indices(4),
        [[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]]
    );
    assert_eq!(
        get_pair_indices(6),
        [
            [0, 1],
            [0, 2],
            [0, 3],
            [0, 4],
            [0, 5],
            [1, 2],
            [1, 3],
            [1, 4],
            [1, 5],
            [2, 3],
            [2, 4],
            [2, 5],
            [3, 4],
            [3, 5],
            [4, 5]
        ]
    );
}

#[test]
fn test_create_tile_connection_lookup_table() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(0), Some(1), Some(2)],
        [Some(3), Some(1), Some(4), Some(2)],
        [Some(4), Some(5), Some(5), Some(0)],
        [Some(0), Some(3), Some(0), Some(0)],
    ]);
    /*
     * 0 0 1 2
     * 3 1 4 2
     * 4 5 5 0
     * 0 3 0 0
     */
    let maplut = create_tile_lookup_table(&map);
    let lut = create_tile_connection_lookup_table(&map, &maplut);
    assert_eq!(lut.len(), 6);
    assert_eq!(
        lut.get(&0),
        Some(&vec![
            ConnectionInfo {
                tile: 0,
                pair: [[0, 0], [0, 1]],
                nodes: [Some([0, 0]), Some([0, 1]), None, None],
            },
            ConnectionInfo {
                tile: 0,
                pair: [[2, 3], [3, 3]],
                nodes: [Some([2, 3]), Some([3, 3]), None, None],
            },
            ConnectionInfo {
                tile: 0,
                pair: [[3, 2], [3, 3]],
                nodes: [Some([3, 2]), Some([3, 3]), None, None],
            },
        ])
    );
    assert_eq!(lut.get(&1), Some(&vec![]));
    assert_eq!(
        lut.get(&2),
        Some(&vec![ConnectionInfo {
            tile: 2,
            pair: [[0, 3], [1, 3]],
            nodes: [Some([0, 3]), Some([1, 3]), None, None],
        }])
    );
    assert_eq!(lut.get(&3), Some(&vec![]));
    assert_eq!(lut.get(&4), Some(&vec![]));
    assert_eq!(
        lut.get(&5),
        Some(&vec![ConnectionInfo {
            tile: 5,
            pair: [[2, 1], [2, 2]],
            nodes: [Some([2, 1]), Some([2, 2]), None, None],
        }])
    );
}
