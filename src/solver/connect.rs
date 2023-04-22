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

fn get_grid_xrange(grid: &Grid) -> [CoordElement; 2] {
    let &[[x1, _], [x2, _]] = grid;
    if (x1 < x2) {
        [x1, x2]
    } else {
        [x2, x1]
    }
}

fn get_grid_yrange(grid: &Grid) -> [CoordElement; 2] {
    let &[[_, y1], [_, y2]] = grid;
    if (y1 < y2) {
        [y1, y2]
    } else {
        [y2, y1]
    }
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

fn explore_horizontal_connection(
    xrange: &[CoordElement; 2],
    yaxis: &[CoordElement; 2],
    map: &TileMap,
) -> Option<Grid> {
    for x in xrange[0]..=xrange[1] {
        match (yaxis[0]..=yaxis[1])
            .map(|y| map.get(&[x, y]) == None)
            .all(|flg| flg)
        {
            true => return Some([[x, yaxis[0]], [x, yaxis[1]]]),
            _ => (),
        }
    }
    None
}

fn explore_vertical_connection(
    yrange: &[CoordElement; 2],
    xaxis: &[CoordElement; 2],
    map: &TileMap,
) -> Option<Grid> {
    for y in yrange[0]..=yrange[1] {
        match (xaxis[0]..=xaxis[1])
            .map(|x| map.get(&[x, y]) == None)
            .all(|flg| flg)
        {
            true => return Some([[xaxis[0], y], [xaxis[1], y]]),
            _ => (),
        }
    }
    None
}

fn is_tile_touched(coord1: &Coord, coord2: &Coord) -> bool {
    match (coord1, coord2) {
        (&[x1, y1], &[x2, y2]) if x1 == x2 => (y1 == y2 + 1) || (y1 + 1 == y2),
        (&[x1, y1], &[x2, y2]) if y1 == y2 => (x1 == x2 + 1) || (x1 + 1 == x2),
        _ => false,
    }
}

fn get_overwrapped_xrange(grid1: &Grid, grid2: &Grid) -> Option<[CoordElement; 2]> {
    let [xmin1, xmax1] = get_grid_xrange(grid1);
    let [xmin2, xmax2] = get_grid_xrange(grid2);
    let xmax = min(xmax1, xmax2);
    let xmin = max(xmin1, xmin2);

    if xmax < xmin {
        None
    } else {
        Some([xmin, xmax])
    }
}

fn get_overwrapped_yrange(grid1: &Grid, grid2: &Grid) -> Option<[CoordElement; 2]> {
    let [ymin1, ymax1] = get_grid_yrange(grid1);
    let [ymin2, ymax2] = get_grid_yrange(grid2);
    let ymax = min(ymax1, ymax2);
    let ymin = max(ymin1, ymin2);

    if ymax < ymin {
        None
    } else {
        Some([ymin, ymax])
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
fn test_get_grid_xrange() {
    let grid1a = [[0, 0], [0, 1]];
    let grid1b = [[0, 1], [0, 0]];
    let grid2a = [[0, 0], [1, 0]];
    let grid2b = [[1, 0], [0, 0]];
    let grid3a = [[0, 0], [1, 1]];
    let grid3b = [[1, 1], [0, 0]];

    assert_eq!(get_grid_xrange(&grid1a), [0, 0]);
    assert_eq!(get_grid_xrange(&grid1b), [0, 0]);
    assert_eq!(get_grid_xrange(&grid2a), [0, 1]);
    assert_eq!(get_grid_xrange(&grid2b), [0, 1]);
    assert_eq!(get_grid_xrange(&grid3a), [0, 1]);
    assert_eq!(get_grid_xrange(&grid3b), [0, 1]);
}

#[test]
fn test_get_grid_yrange() {
    let grid1a = [[0, 0], [0, 1]];
    let grid1b = [[0, 1], [0, 0]];
    let grid2a = [[0, 0], [1, 0]];
    let grid2b = [[1, 0], [0, 0]];
    let grid3a = [[0, 0], [1, 1]];
    let grid3b = [[1, 1], [0, 0]];

    assert_eq!(get_grid_yrange(&grid1a), [0, 1]);
    assert_eq!(get_grid_yrange(&grid1b), [0, 1]);
    assert_eq!(get_grid_yrange(&grid2a), [0, 0]);
    assert_eq!(get_grid_yrange(&grid2b), [0, 0]);
    assert_eq!(get_grid_yrange(&grid3a), [0, 1]);
    assert_eq!(get_grid_yrange(&grid3b), [0, 1]);
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

#[test]
fn test_explore_horizontal_connection() {
    use std::collections::HashMap;
    let map1: TileMap = HashMap::from([([1, 1], 0), ([3, 1], 0)]);
    /* tile map:
     * | x x
     * | 0 |
     * | x |
     * | 0 |
     */
    assert_eq!(
        explore_horizontal_connection(&[1, 3], &[0, 2], &map1),
        Some([[2, 0], [2, 2]])
    );
    assert_eq!(explore_horizontal_connection(&[1, 1], &[0, 2], &map1), None);

    let map2: TileMap = HashMap::from([([1, 1], 0), ([2, 1], 0), ([3, 1], 0)]);
    assert_eq!(explore_horizontal_connection(&[1, 3], &[0, 2], &map2), None);
}

#[test]
fn test_explore_vertical_connection() {
    use std::collections::HashMap;
    let map1: TileMap = HashMap::from([([1, 1], 0), ([1, 3], 0)]);
    /* tile map:
     * -------
     * x 0 x 0
     * x -----
     */
    assert_eq!(
        explore_vertical_connection(&[1, 3], &[0, 2], &map1),
        Some([[0, 2], [2, 2]])
    );
    assert_eq!(explore_vertical_connection(&[1, 1], &[0, 2], &map1), None);

    let map2: TileMap = HashMap::from([([1, 1], 0), ([1, 2], 0), ([1, 3], 0)]);
    assert_eq!(explore_vertical_connection(&[1, 3], &[0, 2], &map2), None);
}

#[test]
fn test_is_tile_touched() {
    assert!(is_tile_touched(&[0, 0], &[1, 0]));
    assert!(is_tile_touched(&[1, 0], &[0, 0]));
    assert!(is_tile_touched(&[1, 1], &[1, 0]));
    assert!(is_tile_touched(&[1, 0], &[1, 1]));
    assert!(!is_tile_touched(&[0, 0], &[1, 1]));
    assert!(!is_tile_touched(&[0, 0], &[0, 2]));
    assert!(!is_tile_touched(&[0, 2], &[0, 0]));
    assert!(!is_tile_touched(&[0, 0], &[2, 0]));
    assert!(!is_tile_touched(&[2, 0], &[0, 0]));
}

#[test]
fn test_get_overwrapped_xrange() {
    let check_all = |grid1: Grid, grid2: Grid, expected: Option<[CoordElement; 2]>| {
        for (g1, g2) in [(grid1, grid2), (grid2, grid1)] {
            for ga in [g1, [g1[1], g1[0]]] {
                for gb in [g2, [g2[1], g2[0]]] {
                    assert_eq!(get_overwrapped_xrange(&ga, &gb), expected);
                }
            }
        }
    };

    check_all([[0, 0], [1, 0]], [[1, 0], [2, 0]], Some([1, 1]));
    check_all([[0, 0], [2, 0]], [[1, 0], [2, 0]], Some([1, 2]));
    check_all([[0, 0], [2, 0]], [[1, 0], [3, 0]], Some([1, 2]));
    check_all([[0, 0], [1, 0]], [[2, 0], [3, 0]], None);
    check_all([[0, 0], [0, 1]], [[1, 2], [2, 2]], None);
    check_all([[0, 0], [0, 1]], [[0, 1], [0, 2]], Some([0, 0]));
    check_all([[0, 0], [0, 2]], [[0, 1], [0, 2]], Some([0, 0]));
    check_all([[0, 0], [0, 2]], [[0, 1], [0, 3]], Some([0, 0]));
    check_all([[0, 0], [0, 1]], [[0, 2], [0, 3]], Some([0, 0]));
}

#[test]
fn test_get_overlapped_yrange() {
    let check_all = |grid1: Grid, grid2: Grid, expected: Option<[CoordElement; 2]>| {
        for (g1, g2) in [(grid1, grid2), (grid2, grid1)] {
            for ga in [g1, [g1[1], g1[0]]] {
                for gb in [g2, [g2[1], g2[0]]] {
                    assert_eq!(get_overwrapped_yrange(&ga, &gb), expected);
                }
            }
        }
    };

    check_all([[0, 0], [1, 0]], [[1, 0], [2, 0]], Some([0, 0]));
    check_all([[0, 0], [2, 0]], [[1, 0], [2, 0]], Some([0, 0]));
    check_all([[0, 0], [2, 0]], [[1, 0], [3, 0]], Some([0, 0]));
    check_all([[0, 0], [1, 0]], [[2, 0], [3, 0]], Some([0, 0]));
    check_all([[0, 0], [0, 1]], [[1, 2], [2, 2]], None);
    check_all([[0, 0], [0, 1]], [[0, 1], [0, 2]], Some([1, 1]));
    check_all([[0, 0], [0, 2]], [[0, 1], [0, 2]], Some([1, 2]));
    check_all([[0, 0], [0, 2]], [[0, 1], [0, 3]], Some([1, 2]));
    check_all([[0, 0], [0, 1]], [[0, 2], [0, 3]], None);
}
