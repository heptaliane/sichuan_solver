use super::super::components::{Coord, CoordDelta, CoordDeltaElement, CoordElement, Grid, TileMap};
use std::cmp::max;

fn get_size_from_map(map: &TileMap) -> [usize; 2] {
    map.keys()
        .fold([0, 0], |[mx, my], &[x, y]| [max(x, mx), max(y, my)])
        .map(|v| v + 1)
}

fn move_coord(coord: &Coord, delta: &CoordDelta, map_size: &[usize; 2]) -> Option<Coord> {
    let moved_coord = [
        coord[0] as CoordDeltaElement + delta[0],
        coord[1] as CoordDeltaElement + delta[1],
    ];

    match moved_coord {
        [x, y] if x < 0 || y < 0 => None,
        [x, y] if x as usize >= map_size[0] || y as usize >= map_size[1] => None,
        [x, y] => Some([x as CoordElement, y as CoordElement]),
    }
}

fn get_grid(
    &coord: &Coord,
    delta: &CoordDelta,
    map: &TileMap,
    map_size: &[usize; 2],
) -> Option<Grid> {
    let mut cursor = coord.clone();
    let mut next_cursor = move_coord(&coord, delta, map_size);

    loop {
        match next_cursor {
            Some(crd) => match map.get(&crd) {
                None => {
                    cursor = next_cursor.unwrap();
                    next_cursor = move_coord(&cursor, delta, map_size);
                }
                _ => break,
            },
            _ => break,
        }
    }

    match coord == cursor {
        true => None,
        false => Some([coord, cursor]),
    }
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

#[test]
fn test_move_coord() {
    let map_size = [3, 4];
    let move_up = [-1, 0];
    let move_down = [1, 0];
    let move_left = [0, -1];
    let move_right = [0, 1];

    assert_eq!(move_coord(&[1, 1], &move_down, &map_size), Some([2, 1]));
    assert_eq!(move_coord(&[1, 1], &move_right, &map_size), Some([1, 2]));
    assert_eq!(move_coord(&[1, 1], &move_up, &map_size), Some([0, 1]));
    assert_eq!(move_coord(&[1, 1], &move_left, &map_size), Some([1, 0]));
    assert_eq!(move_coord(&[0, 0], &move_up, &map_size), None);
    assert_eq!(move_coord(&[0, 0], &move_left, &map_size), None);
}

#[test]
fn test_get_grid() {
    use std::collections::HashMap;
    /* tile map:
     * x 0 x 0
     * 0 x x x
     * x x x 0
     */
    let map: TileMap = HashMap::from([([0, 1], 0), ([0, 3], 0), ([1, 0], 0), ([2, 3], 0)]);
    let map_size = get_size_from_map(&map);
    let move_up = [-1, 0];
    let move_down = [1, 0];
    let move_left = [0, -1];
    let move_right = [0, 1];

    assert_eq!(get_grid(&[0, 2], &move_left, &map, &map_size), None);
    assert_eq!(get_grid(&[0, 2], &move_right, &map, &map_size), None);
    assert_eq!(get_grid(&[1, 3], &move_up, &map, &map_size), None);
    assert_eq!(get_grid(&[1, 3], &move_down, &map, &map_size), None);
    assert_eq!(
        get_grid(&[0, 3], &move_left, &map, &map_size),
        Some([[0, 3], [0, 2]])
    );
    assert_eq!(
        get_grid(&[0, 1], &move_right, &map, &map_size),
        Some([[0, 1], [0, 2]])
    );
    assert_eq!(
        get_grid(&[2, 3], &move_up, &map, &map_size),
        Some([[2, 3], [1, 3]])
    );
    assert_eq!(
        get_grid(&[0, 3], &move_down, &map, &map_size),
        Some([[0, 3], [1, 3]])
    );
    assert_eq!(
        get_grid(&[1, 2], &move_left, &map, &map_size),
        Some([[1, 2], [1, 1]])
    );
    assert_eq!(
        get_grid(&[1, 2], &move_right, &map, &map_size),
        Some([[1, 2], [1, 3]])
    );
    assert_eq!(
        get_grid(&[1, 2], &move_up, &map, &map_size),
        Some([[1, 2], [0, 2]])
    );
    assert_eq!(
        get_grid(&[1, 2], &move_down, &map, &map_size),
        Some([[1, 2], [2, 2]])
    );
}
