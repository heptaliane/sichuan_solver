use std::cmp::{max, min};

use super::super::components::{
    Coord, CoordDelta, CoordDeltaElement, CoordElement, Grid, Nodes, TileMap,
};
use super::direction::{DOWN, LEFT, RIGHT, UP};

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

fn get_grid_pair(
    coord1: &Coord,
    coord2: &Coord,
    direction1: &CoordDelta,
    direction2: &CoordDelta,
    map: &TileMap,
    map_size: &[usize; 2],
) -> Option<[Grid; 2]> {
    let grid1 = get_grid(coord1, direction1, map, map_size);
    let grid2 = get_grid(coord2, direction2, map, map_size);
    match (grid1, grid2) {
        (Some(g1), Some(g2)) => Some([g1, g2]),
        _ => None,
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
    if x1 < x2 {
        [x1, x2]
    } else {
        [x2, x1]
    }
}

fn get_grid_yrange(grid: &Grid) -> [CoordElement; 2] {
    let &[[_, y1], [_, y2]] = grid;
    if y1 < y2 {
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

fn can_horizontal_connect(
    y1: CoordElement,
    y2: CoordElement,
    x: CoordElement,
    map: &TileMap,
) -> bool {
    (y1..=y2).all(|y| map.get(&[x, y]) == None)
}

fn explore_horizontal_connection(
    xrange: &[CoordElement; 2],
    yaxis: &[CoordElement; 2],
    map: &TileMap,
) -> Option<Grid> {
    for x in xrange[0]..=xrange[1] {
        if can_horizontal_connect(yaxis[0], yaxis[1], x, map) {
            return Some([[x, yaxis[0]], [x, yaxis[1]]]);
        }
    }
    None
}

fn can_vertical_connect(
    x1: CoordElement,
    x2: CoordElement,
    y: CoordElement,
    map: &TileMap,
) -> bool {
    (x1..=x2).all(|x| map.get(&[x, y]) == None)
}

fn explore_vertical_connection(
    yrange: &[CoordElement; 2],
    xaxis: &[CoordElement; 2],
    map: &TileMap,
) -> Option<Grid> {
    for y in yrange[0]..=yrange[1] {
        if can_vertical_connect(xaxis[0], xaxis[1], y, map) {
            return Some([[xaxis[0], y], [xaxis[1], y]]);
        }
    }
    None
}

fn is_tile_adjacent(coord1: &Coord, coord2: &Coord) -> bool {
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

fn is_connected_with_single_line(grid1: &Grid, grid2: &Grid) -> bool {
    match (grid1, grid2) {
        (g1, g2) if is_horizonal(g1) && is_horizonal(g2) && g1[0][0] == g2[0][0] => {
            get_overwrapped_yrange(g1, g2) != None
        }
        (g1, g2) if is_vertical(g1) && is_vertical(g2) && g1[0][1] == g2[0][1] => {
            get_overwrapped_xrange(g1, g2) != None
        }
        _ => false,
    }
}

fn try_get_double_node_connection(
    &coord1: &Coord,
    &coord2: &Coord,
    map: &TileMap,
) -> Option<Nodes> {
    let nodes = vec![coord1, coord2];
    if is_tile_adjacent(&coord1, &coord2) {
        return Some(nodes);
    }

    if coord1[0] == coord2[0] && can_horizontal_connect(coord1[1], coord2[1], coord1[0], map) {
        return Some(nodes);
    }

    if coord1[1] == coord2[1] && can_vertical_connect(coord1[0], coord2[0], coord1[1], map) {
        return Some(nodes);
    }

    None
}

fn try_get_triple_node_connection(
    coord1: &Coord,
    coord2: &Coord,
    map: &TileMap,
    map_size: &[usize; 2],
) -> Option<Nodes> {
    let (v_dir1, v_dir2) = match coord1[0] > coord2[0] {
        true => (UP, DOWN),
        false => (DOWN, UP),
    };
    let (h_dir1, h_dir2) = match coord1[1] > coord2[1] {
        true => (LEFT, RIGHT),
        false => (RIGHT, LEFT),
    };

    for (dir1, dir2) in [(v_dir1, h_dir2), (h_dir1, v_dir2)].iter() {
        if let Some([grid1, grid2]) = get_grid_pair(coord1, coord2, dir1, dir2, map, map_size) {
            if let Some(intersection) = get_intersection(&grid1, &grid2) {
                return Some(vec![coord1.to_owned(), intersection, coord2.to_owned()]);
            }
        }
    }

    None
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

    assert_eq!(move_coord(&[1, 1], &DOWN, &map_size), Some([2, 1]));
    assert_eq!(move_coord(&[1, 1], &RIGHT, &map_size), Some([1, 2]));
    assert_eq!(move_coord(&[1, 1], &UP, &map_size), Some([0, 1]));
    assert_eq!(move_coord(&[1, 1], &LEFT, &map_size), Some([1, 0]));
    assert_eq!(move_coord(&[0, 0], &UP, &map_size), None);
    assert_eq!(move_coord(&[0, 0], &LEFT, &map_size), None);
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

    assert_eq!(get_grid(&[0, 2], &LEFT, &map, &map_size), None);
    assert_eq!(get_grid(&[0, 2], &RIGHT, &map, &map_size), None);
    assert_eq!(get_grid(&[1, 3], &UP, &map, &map_size), None);
    assert_eq!(get_grid(&[1, 3], &DOWN, &map, &map_size), None);
    assert_eq!(
        get_grid(&[0, 3], &LEFT, &map, &map_size),
        Some([[0, 3], [0, 2]])
    );
    assert_eq!(
        get_grid(&[0, 1], &RIGHT, &map, &map_size),
        Some([[0, 1], [0, 2]])
    );
    assert_eq!(
        get_grid(&[2, 3], &UP, &map, &map_size),
        Some([[2, 3], [1, 3]])
    );
    assert_eq!(
        get_grid(&[0, 3], &DOWN, &map, &map_size),
        Some([[0, 3], [1, 3]])
    );
    assert_eq!(
        get_grid(&[1, 2], &LEFT, &map, &map_size),
        Some([[1, 2], [1, 1]])
    );
    assert_eq!(
        get_grid(&[1, 2], &RIGHT, &map, &map_size),
        Some([[1, 2], [1, 3]])
    );
    assert_eq!(
        get_grid(&[1, 2], &UP, &map, &map_size),
        Some([[1, 2], [0, 2]])
    );
    assert_eq!(
        get_grid(&[1, 2], &DOWN, &map, &map_size),
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
    assert!(is_tile_adjacent(&[0, 0], &[1, 0]));
    assert!(is_tile_adjacent(&[1, 0], &[0, 0]));
    assert!(is_tile_adjacent(&[1, 1], &[1, 0]));
    assert!(is_tile_adjacent(&[1, 0], &[1, 1]));
    assert!(!is_tile_adjacent(&[0, 0], &[1, 1]));
    assert!(!is_tile_adjacent(&[0, 0], &[0, 2]));
    assert!(!is_tile_adjacent(&[0, 2], &[0, 0]));
    assert!(!is_tile_adjacent(&[0, 0], &[2, 0]));
    assert!(!is_tile_adjacent(&[2, 0], &[0, 0]));
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

#[test]
fn test_is_connect_with_single_line() {
    assert!(is_connected_with_single_line(
        &[[0, 0], [0, 2]],
        &[[0, 1], [0, 3]]
    ));
    assert!(is_connected_with_single_line(
        &[[0, 0], [0, 2]],
        &[[0, 2], [0, 3]]
    ));
    assert!(!is_connected_with_single_line(
        &[[0, 0], [0, 2]],
        &[[1, 1], [1, 3]]
    ));
    assert!(!is_connected_with_single_line(
        &[[0, 0], [0, 1]],
        &[[0, 2], [0, 3]]
    ));
    assert!(is_connected_with_single_line(
        &[[0, 0], [2, 0]],
        &[[1, 0], [3, 0]]
    ));
    assert!(is_connected_with_single_line(
        &[[0, 0], [2, 0]],
        &[[2, 0], [3, 0]]
    ));
    assert!(!is_connected_with_single_line(
        &[[0, 0], [2, 0]],
        &[[1, 1], [3, 1]]
    ));
    assert!(!is_connected_with_single_line(
        &[[0, 0], [1, 0]],
        &[[2, 0], [3, 0]]
    ));
    assert!(!is_connected_with_single_line(
        &[[0, 0], [1, 0]],
        &[[0, 0], [0, 1]]
    ));
    assert!(!is_connected_with_single_line(
        &[[0, 0], [2, 0]],
        &[[1, 0], [1, 1]]
    ));
    assert!(!is_connected_with_single_line(
        &[[0, 1], [1, 1]],
        &[[0, 0], [1, 2]]
    ));
    assert!(!is_connected_with_single_line(
        &[[0, 1], [0, 2]],
        &[[0, 0], [1, 0]]
    ));
}

#[test]
fn test_try_get_double_node_connection() {
    use std::collections::HashMap;
    /*
     * x x x
     * x 0 x
     * x x x
     */
    let map: TileMap = HashMap::from([([1, 1], 0)]);

    assert_eq!(
        try_get_double_node_connection(&[0, 0], &[0, 2], &map),
        Some(vec![[0, 0], [0, 2]])
    );
    assert_eq!(
        try_get_double_node_connection(&[0, 0], &[2, 0], &map),
        Some(vec![[0, 0], [2, 0]])
    );
    assert_eq!(
        try_get_double_node_connection(&[0, 0], &[0, 1], &map),
        Some(vec![[0, 0], [0, 1]])
    );
    assert_eq!(
        try_get_double_node_connection(&[0, 0], &[1, 0], &map),
        Some(vec![[0, 0], [1, 0]])
    );
    assert_eq!(try_get_double_node_connection(&[1, 0], &[1, 2], &map), None);
    assert_eq!(try_get_double_node_connection(&[0, 1], &[2, 1], &map), None);
    assert_eq!(try_get_double_node_connection(&[0, 0], &[2, 2], &map), None);
}

#[test]
fn test_try_get_triple_node_connection() {
    use std::collections::HashMap;
    /*
     * x x x
     * x 0 x
     * x x 0
     */
    let map1: TileMap = HashMap::from([([1, 1], 0), ([2, 2], 0)]);
    let map_size = [3, 3];

    assert_eq!(
        try_get_triple_node_connection(&[0, 1], &[1, 0], &map1, &map_size),
        Some(vec![[0, 1], [0, 0], [1, 0]])
    );
    assert_eq!(
        try_get_triple_node_connection(&[0, 2], &[2, 0], &map1, &map_size),
        Some(vec![[0, 2], [0, 0], [2, 0]])
    );
    assert_eq!(
        try_get_triple_node_connection(&[1, 2], &[2, 1], &map1, &map_size),
        None
    );

    /*
     * 0 x x
     * x 0 x
     * x x x
     */
    let map2: TileMap = HashMap::from([([0, 0], 0), ([1, 1], 0)]);
    assert_eq!(
        try_get_triple_node_connection(&[0, 1], &[1, 0], &map2, &map_size),
        None
    );
    assert_eq!(
        try_get_triple_node_connection(&[0, 2], &[2, 0], &map2, &map_size),
        Some(vec![[0, 2], [2, 2], [2, 0]])
    );
    assert_eq!(
        try_get_triple_node_connection(&[1, 2], &[2, 1], &map2, &map_size),
        Some(vec![[1, 2], [2, 2], [2, 1]])
    );
}
