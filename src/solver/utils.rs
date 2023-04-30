use super::super::components::{Coord, Nodes, TileMap};
use std::cmp::max;

pub fn pad_tilemap(map: &TileMap) -> TileMap {
    map.iter().map(|([x, y], &t)| ([x + 1, y + 1], t)).collect()
}

pub fn get_size_from_map(map: &TileMap) -> [usize; 2] {
    map.keys()
        .fold([0, 0], |[mx, my], &[x, y]| [max(x, mx), max(y, my)])
        .map(|v| v + 1)
}

pub fn get_node_edges(node: &Nodes) -> [Coord; 2] {
    [node.first().unwrap().clone(), node.last().unwrap().clone()]
}

#[test]
fn test_pad_tilemap() {
    use std::collections::HashMap;
    let map: TileMap = HashMap::from([
        ([0, 0], 0),
        ([0, 1], 1),
        ([0, 2], 2),
        ([1, 0], 2),
        ([1, 1], 0),
        ([1, 2], 1),
    ]);
    let expected: TileMap = HashMap::from([
        ([1, 1], 0),
        ([1, 2], 1),
        ([1, 3], 2),
        ([2, 1], 2),
        ([2, 2], 0),
        ([2, 3], 1),
    ]);
    assert_eq!(expected, pad_tilemap(&map));
}

#[test]
fn test_get_map_size() {
    use std::collections::HashMap;
    /* tile map:
     * x 0 x 0
     * 0 x x x
     * x x 0 x
     */
    let map: TileMap = HashMap::from([([0, 1], 0), ([0, 3], 0), ([1, 0], 0), ([2, 2], 0)]);
    let expected = [3, 4];
    assert_eq!(get_size_from_map(&map), expected);
}
