use super::components::{Coord, Nodes, Tile, TileMap};
use super::connect::find_connection;
use std::collections::HashMap;
use std::iter::FromIterator;

type ConnetionList = HashMap<[Coord; 2], Nodes>;
type TileLut = HashMap<Tile, Vec<Coord>>;

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

fn collect_connectable_single_pair_tiles(map: &TileMap) -> ConnetionList {
    let lut = create_tile_lookup_table(map);
    let mut connectables: ConnetionList = HashMap::new();

    for coords in lut.values().filter(|coord| coord.len() == 2) {
        match find_connection(map, &coords[0], &coords[1]) {
            Some(nodes) => {
                connectables.insert([coords[0], coords[1]], nodes);
            }
            None => (),
        }
    }
    connectables
}

fn remove_connected_tiles(map: &TileMap, connected: &ConnetionList) -> TileMap {
    let mut new_map = map.clone();
    for coords in connected.keys() {
        for &coord in coords {
            new_map[coord] = None;
        }
    }
    new_map
}

fn collect_connectable_single_pair_tiles_loop(map: &TileMap) -> ConnetionList {
    let mut connectables = collect_connectable_single_pair_tiles(map);
    let mut current_map = remove_connected_tiles(map, &connectables);

    loop {
        let conn = collect_connectable_single_pair_tiles(&current_map);
        if conn.len() == 0 {
            break;
        }

        current_map = remove_connected_tiles(&current_map, &conn);
        connectables.extend(conn);
    }
    connectables
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
fn test_collect_connectable_single_pair_tiles() {
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
    let connectables = collect_connectable_single_pair_tiles(&map);
    assert_eq!(
        connectables.get(&[[0, 2], [1, 1]]),
        Some(&[Some([0, 2]), Some([1, 2]), Some([1, 1]), None])
    );
    assert_eq!(
        connectables.get(&[[0, 3], [1, 3]]),
        Some(&[Some([0, 3]), Some([1, 3]), None, None])
    );
    assert_eq!(
        connectables.get(&[[1, 0], [3, 1]]),
        Some(&[Some([1, 0]), Some([2, 0]), Some([2, 1]), Some([3, 1])])
    );
    assert_eq!(connectables.len(), 3);
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
    let connectables = collect_connectable_single_pair_tiles(&map);
    assert_eq!(remove_connected_tiles(&map, &connectables), map2);
}

#[test]
fn test_collect_connectable_single_pair_tiles_loop() {
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
    let connectables = collect_connectable_single_pair_tiles_loop(&map);
    assert_eq!(connectables.len(), 5);
    assert_eq!(
        connectables.get(&[[0, 2], [1, 1]]),
        Some(&[Some([0, 2]), Some([1, 2]), Some([1, 1]), None])
    );
    assert_eq!(
        connectables.get(&[[0, 3], [1, 3]]),
        Some(&[Some([0, 3]), Some([1, 3]), None, None])
    );
    assert_eq!(
        connectables.get(&[[2, 1], [2, 2]]),
        Some(&[Some([2, 1]), Some([2, 2]), None, None])
    );
    assert_eq!(
        connectables.get(&[[1, 2], [2, 0]]),
        Some(&[Some([1, 2]), Some([2, 2]), Some([2, 0]), None])
    );
    assert_eq!(
        connectables.get(&[[1, 0], [3, 1]]),
        Some(&[Some([1, 0]), Some([2, 0]), Some([2, 1]), Some([3, 1])])
    );
}
