use super::super::components::TileMap;

fn pad_tilemap(map: &TileMap) -> TileMap {
    map.iter().map(|([x, y], &t)| ([x + 1, y + 1], t)).collect()
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
