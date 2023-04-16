use super::super::components::{Coord, CoordDelta, CoordDeltaElement, CoordElement, Grid, TileMap};
use std::cmp::{max, min};

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

fn is_horizonal(grid: &Grid) -> bool {
    grid[0][0] == grid[1][0]
}

fn is_vertical(grid: &Grid) -> bool {
    grid[0][1] == grid[1][1]
}

fn get_intersection(grid1: &Grid, grid2: &Grid) -> Option<Coord> {
    match (grid1, grid2) {
        (g1, g2) if is_horizonal(g1) && is_vertical(g2) => match (g1[0][0], g2[0][1]) {
            (x, _) if max(g2[0][0], g2[1][0]) < x => None,
            (x, _) if min(g2[0][0], g2[1][0]) > x => None,
            (_, y) if max(g1[0][1], g1[1][1]) < y => None,
            (_, y) if min(g1[0][1], g1[1][1]) > y => None,
            (x, y) => Some([x, y]),
        },
        (g1, g2) if is_vertical(g1) && is_horizonal(g2) => match (g2[0][0], g1[0][1]) {
            (x, _) if max(g1[0][0], g1[1][0]) < x => None,
            (x, _) if min(g1[0][0], g1[1][0]) > x => None,
            (_, y) if max(g2[0][1], g2[1][1]) < y => None,
            (_, y) if min(g2[0][1], g2[1][1]) > y => None,
            (x, y) => Some([x, y]),
        },
        _ => None,
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

#[test]
fn test_is_horizontal() {
    let grid1 = [[0, 0], [0, 1]];
    let grid2 = [[0, 0], [1, 0]];
    let grid3 = [[0, 0], [1, 1]];

    assert_eq!(is_horizonal(&grid1), true);
    assert_eq!(is_horizonal(&grid2), false);
    assert_eq!(is_horizonal(&grid3), false);
}

#[test]
fn test_is_vertical() {
    let grid1 = [[0, 0], [0, 1]];
    let grid2 = [[0, 0], [1, 0]];
    let grid3 = [[0, 0], [1, 1]];

    assert_eq!(is_vertical(&grid1), false);
    assert_eq!(is_vertical(&grid2), true);
    assert_eq!(is_vertical(&grid3), false);
}

#[test]
fn test_get_intersection() {
    let check_all = |grid1: Grid, grid2: Grid, expected: Option<Coord>| {
        for [g1, g2] in [[grid1, grid2], [grid2, grid1]] {
            for ga in [g1, [g1[1], g1[0]]] {
                for gb in [g2, [g2[1], g2[0]]] {
                    assert_eq!(get_intersection(&ga, &gb), expected);
                }
            }
        }
    };

    check_all([[0, 0], [0, 1]], [[0, 0], [1, 0]], Some([0, 0]));
    check_all([[1, 1], [0, 1]], [[1, 1], [1, 0]], Some([1, 1]));
    check_all([[0, 0], [0, 2]], [[0, 1], [1, 1]], Some([0, 1]));
    check_all([[1, 0], [1, 2]], [[0, 1], [1, 1]], Some([1, 1]));
    check_all([[0, 0], [0, 1]], [[0, 2], [1, 2]], None);
    check_all([[2, 2], [2, 1]], [[0, 0], [2, 0]], None);
    check_all([[0, 0], [0, 2]], [[1, 1], [2, 1]], None);
    check_all([[2, 0], [2, 2]], [[0, 1], [1, 1]], None);
    check_all([[1, 0], [1, 1]], [[0, 0], [2, 0]], Some([1, 0]));
    check_all([[1, 0], [1, 1]], [[0, 1], [2, 1]], Some([1, 1]));
    check_all([[1, 0], [1, 2]], [[0, 1], [2, 1]], Some([1, 1]));
    check_all([[0, 0], [0, 2]], [[0, 1], [0, 3]], None);
    check_all([[0, 0], [0, 2]], [[1, 1], [1, 3]], None);
    check_all([[0, 0], [2, 0]], [[1, 0], [3, 0]], None);
    check_all([[0, 0], [2, 0]], [[1, 1], [3, 1]], None);
}
