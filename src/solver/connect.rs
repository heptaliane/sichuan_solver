use std::cmp::{max, min};

use super::super::components::{
    Coord, CoordDelta, CoordDeltaElement, CoordElement, Grid, Nodes, TileMap,
};
use super::direction::{DOWN, LEFT, RIGHT, UP};

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

    match x1 < x2 {
        true => [x1, x2],
        false => [x2, x1],
    }
}

fn get_grid_yrange(grid: &Grid) -> [CoordElement; 2] {
    let &[[_, y1], [_, y2]] = grid;

    match y1 < y2 {
        true => [y1, y2],
        false => [y2, y1],
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
    ignore_edge: bool,
    map: &TileMap,
) -> bool {
    let (ymin, ymax) = match y1 < y2 {
        true => (y1, y2),
        false => (y2, y1),
    };
    match ignore_edge {
        true => (ymin + 1)..ymax,
        false => ymin..(ymax + 1),
    }
    .all(|y| !map.contains_key(&[x, y]))
}

fn explore_horizontal_connection(
    xrange: &[CoordElement; 2],
    yaxis: &[CoordElement; 2],
    map: &TileMap,
) -> Option<Grid> {
    for x in xrange[0]..=xrange[1] {
        if can_horizontal_connect(yaxis[0], yaxis[1], x, false, map) {
            return Some([[x, yaxis[0]], [x, yaxis[1]]]);
        }
    }
    None
}

fn can_vertical_connect(
    x1: CoordElement,
    x2: CoordElement,
    y: CoordElement,
    ignore_edge: bool,
    map: &TileMap,
) -> bool {
    let (xmin, xmax) = match x1 < x2 {
        true => (x1, x2),
        false => (x2, x1),
    };
    match ignore_edge {
        true => (xmin + 1)..xmax,
        false => xmin..(xmax + 1),
    }
    .all(|x| !map.contains_key(&[x, y]))
}

fn explore_vertical_connection(
    yrange: &[CoordElement; 2],
    xaxis: &[CoordElement; 2],
    map: &TileMap,
) -> Option<Grid> {
    for y in yrange[0]..=yrange[1] {
        if can_vertical_connect(xaxis[0], xaxis[1], y, false, map) {
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

fn try_get_double_node_connection(
    &coord1: &Coord,
    &coord2: &Coord,
    map: &TileMap,
) -> Option<Nodes> {
    let nodes = vec![coord1, coord2];
    if is_tile_adjacent(&coord1, &coord2) {
        return Some(nodes);
    }

    if coord1[0] == coord2[0] && can_horizontal_connect(coord1[1], coord2[1], coord1[0], true, map)
    {
        return Some(nodes);
    }

    if coord1[1] == coord2[1] && can_vertical_connect(coord1[0], coord2[0], coord1[1], true, map) {
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

fn try_get_quadro_node_connection(
    coord1: &Coord,
    coord2: &Coord,
    map: &TileMap,
    map_size: &[usize; 2],
) -> Option<Nodes> {
    let xaxis = [coord1[0], coord2[0]];
    let yaxis = [coord1[1], coord2[1]];

    /*
     * | x |   o x x
     * o x |   | x o
     * x x o   | x |
     */
    for direction in [UP, DOWN].iter() {
        let grids = get_grid_pair(coord1, coord2, direction, direction, map, map_size);
        if let Some([grid1, grid2]) = grids {
            if let Some(xrange) = get_overwrapped_xrange(&grid1, &grid2) {
                let grid = explore_horizontal_connection(&xrange, &yaxis, map);
                if let Some([coord1b, coord2b]) = grid {
                    return Some(vec![coord1.to_owned(), coord1b, coord2b, coord2.to_owned()]);
                }
            }
        }
    }

    /*
     * o - -   - - o
     * x x x   x x x
     * x o -   - o x
     */
    for direction in [LEFT, RIGHT].iter() {
        let grids = get_grid_pair(coord1, coord2, direction, direction, map, map_size);
        if let Some([grid1, grid2]) = grids {
            if let Some(yrange) = get_overwrapped_yrange(&grid1, &grid2) {
                let grid = explore_vertical_connection(&yrange, &xaxis, map);
                if let Some([coord1b, coord2b]) = grid {
                    return Some(vec![coord1.to_owned(), coord1b, coord2b, coord2.to_owned()]);
                }
            }
        }
    }

    /*
     * | x o
     * | x |
     * o x |
     */
    if let Some([grid1, grid2]) = match coord1[0] > coord2[0] {
        true => get_grid_pair(coord1, coord2, &UP, &DOWN, map, map_size),
        false => get_grid_pair(coord1, coord2, &DOWN, &UP, map, map_size),
    } {
        if let Some(xrange) = get_overwrapped_xrange(&grid1, &grid2) {
            let grid = explore_horizontal_connection(&xrange, &yaxis, map);
            if let Some([coord1b, coord2b]) = grid {
                return Some(vec![coord1.to_owned(), coord1b, coord2b, coord2.to_owned()]);
            }
        }
    }

    /*
     * - - o
     * x x x
     * o - -
     */
    if let Some([grid1, grid2]) = match coord1[1] > coord2[1] {
        true => get_grid_pair(coord1, coord2, &LEFT, &RIGHT, map, map_size),
        false => get_grid_pair(coord1, coord2, &RIGHT, &LEFT, map, map_size),
    } {
        if let Some(yrange) = get_overwrapped_yrange(&grid1, &grid2) {
            let grid = explore_vertical_connection(&yrange, &xaxis, map);
            if let Some([coord1b, coord2b]) = grid {
                return Some(vec![coord1.to_owned(), coord1b, coord2b, coord2.to_owned()]);
            }
        }
    }

    None
}

pub fn try_get_node_connection(
    coord1: &Coord,
    coord2: &Coord,
    map: &TileMap,
    map_size: &[usize; 2],
) -> Option<Nodes> {
    let double_node_connection = try_get_double_node_connection(coord1, coord2, map);
    if double_node_connection != None {
        return double_node_connection;
    }

    let triple_node_connection = try_get_triple_node_connection(coord1, coord2, map, map_size);
    if triple_node_connection != None {
        return triple_node_connection;
    }

    try_get_quadro_node_connection(coord1, coord2, map, map_size)
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
    let map_size = [3, 4];

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
    assert_eq!(
        try_get_triple_node_connection(&[1, 1], &[2, 2], &map1, &map_size),
        Some(vec![[1, 1], [2, 1], [2, 2]])
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

#[test]
fn test_try_get_quadro_node_connection() {
    use std::collections::HashMap;
    let map_size = [3, 3];

    let reverse = |nodes: Option<Nodes>| match nodes {
        Some(arr) => {
            let mut reversed = arr.clone();
            reversed.reverse();
            Some(reversed)
        }
        None => None,
    };

    let map1 = HashMap::from([([0, 0], 0), ([0, 1], 1), ([1, 1], 1), ([1, 2], 0)]);
    let (coord1a, coord1b) = ([0, 0], [1, 2]);
    let expected1 = Some(vec![[0, 0], [2, 0], [2, 2], [1, 2]]);
    assert_eq!(
        try_get_quadro_node_connection(&coord1a, &coord1b, &map1, &map_size),
        expected1
    );
    assert_eq!(
        try_get_quadro_node_connection(&coord1b, &coord1a, &map1, &map_size),
        reverse(expected1)
    );

    let map2 = HashMap::from([([1, 1], 1), ([1, 2], 0), ([2, 0], 0), ([2, 1], 1)]);
    let (coord2a, coord2b) = ([2, 0], [1, 2]);
    let expected2 = Some(vec![[2, 0], [0, 0], [0, 2], [1, 2]]);
    assert_eq!(
        try_get_quadro_node_connection(&coord2a, &coord2b, &map2, &map_size),
        expected2
    );
    assert_eq!(
        try_get_quadro_node_connection(&coord2b, &coord2a, &map2, &map_size),
        reverse(expected2)
    );

    let map3 = HashMap::from([([0, 0], 0), ([1, 0], 1), ([1, 1], 1), ([2, 1], 0)]);
    let (coord3a, coord3b) = ([0, 0], [2, 1]);
    let expected3 = Some(vec![[0, 0], [0, 2], [2, 2], [2, 1]]);
    assert_eq!(
        try_get_quadro_node_connection(&coord3a, &coord3b, &map3, &map_size),
        expected3
    );
    assert_eq!(
        try_get_quadro_node_connection(&coord3b, &coord3a, &map3, &map_size),
        reverse(expected3)
    );

    let map4 = HashMap::from([([0, 2], 0), ([1, 1], 1), ([1, 2], 1), ([2, 1], 0)]);
    let (coord4a, coord4b) = ([0, 2], [2, 1]);
    let expected4 = Some(vec![[0, 2], [0, 0], [2, 0], [2, 1]]);
    assert_eq!(
        try_get_quadro_node_connection(&coord4a, &coord4b, &map4, &map_size),
        expected4
    );
    assert_eq!(
        try_get_quadro_node_connection(&coord4b, &coord4a, &map4, &map_size),
        reverse(expected4)
    );

    let map5 = HashMap::from([([0, 2], 0), ([1, 0], 1), ([1, 2], 1), ([2, 0], 0)]);
    let (coord5a, coord5b) = ([0, 2], [2, 0]);
    let expected5 = Some(vec![[0, 2], [0, 1], [2, 1], [2, 0]]);
    assert_eq!(
        try_get_quadro_node_connection(&coord5a, &coord5b, &map5, &map_size),
        expected5
    );
    assert_eq!(
        try_get_quadro_node_connection(&coord5b, &coord5a, &map5, &map_size),
        reverse(expected5)
    );

    let map6 = HashMap::from([([0, 0], 0), ([1, 0], 1), ([1, 2], 1), ([2, 2], 0)]);
    let (coord6a, coord6b) = ([0, 0], [2, 2]);
    let expected6 = Some(vec![[0, 0], [0, 1], [2, 1], [2, 2]]);
    assert_eq!(
        try_get_quadro_node_connection(&coord6a, &coord6b, &map6, &map_size),
        expected6
    );
    assert_eq!(
        try_get_quadro_node_connection(&coord6b, &coord6a, &map6, &map_size),
        reverse(expected6)
    );

    let map7 = HashMap::from([([0, 2], 0), ([0, 1], 1), ([2, 1], 1), ([2, 0], 0)]);
    let (coord7a, coord7b) = ([0, 2], [2, 0]);
    let expected7 = Some(vec![[0, 2], [1, 2], [1, 0], [2, 0]]);
    assert_eq!(
        try_get_quadro_node_connection(&coord7a, &coord7b, &map7, &map_size),
        expected7
    );
    assert_eq!(
        try_get_quadro_node_connection(&coord7b, &coord7a, &map7, &map_size),
        reverse(expected7)
    );

    let map8 = HashMap::from([([0, 0], 0), ([0, 1], 1), ([2, 1], 1), ([2, 2], 0)]);
    let (coord8a, coord8b) = ([0, 0], [2, 2]);
    let expected8 = Some(vec![[0, 0], [1, 0], [1, 2], [2, 2]]);
    assert_eq!(
        try_get_quadro_node_connection(&coord8a, &coord8b, &map8, &map_size),
        expected8
    );
    assert_eq!(
        try_get_quadro_node_connection(&coord8b, &coord8a, &map8, &map_size),
        reverse(expected8)
    );

    let map9 = HashMap::from([([0, 0], 1), ([0, 1], 0), ([1, 0], 0), ([1, 1], 1)]);
    let (coord9a, coord9b) = ([0, 0], [1, 1]);
    assert_eq!(
        try_get_quadro_node_connection(&coord9a, &coord9b, &map9, &map_size),
        None
    );
    assert_eq!(
        try_get_quadro_node_connection(&coord9b, &coord9a, &map9, &map_size),
        None
    );

    let map10 = HashMap::from([([0, 1], 0), ([0, 2], 1), ([1, 1], 1), ([1, 2], 0)]);
    let (coord10a, coord10b) = ([0, 1], [1, 2]);
    assert_eq!(
        try_get_quadro_node_connection(&coord10a, &coord10b, &map10, &map_size),
        None
    );
    assert_eq!(
        try_get_quadro_node_connection(&coord10b, &coord10a, &map10, &map_size),
        None
    );

    let map11 = HashMap::from([([1, 0], 0), ([1, 1], 1), ([2, 0], 1), ([2, 1], 0)]);
    let (coord11a, coord11b) = ([1, 0], [2, 1]);
    assert_eq!(
        try_get_quadro_node_connection(&coord11a, &coord11b, &map11, &map_size),
        None
    );
    assert_eq!(
        try_get_quadro_node_connection(&coord11b, &coord11a, &map11, &map_size),
        None
    );

    let map12 = HashMap::from([([1, 1], 1), ([1, 2], 0), ([2, 1], 0), ([2, 2], 1)]);
    let (coord12a, coord12b) = ([1, 2], [2, 1]);
    assert_eq!(
        try_get_quadro_node_connection(&coord12a, &coord12b, &map12, &map_size),
        None
    );
    assert_eq!(
        try_get_quadro_node_connection(&coord12b, &coord12a, &map12, &map_size),
        None
    );
}

#[test]
fn test_try_get_node_connection() {
    use std::collections::HashMap;
    /*
     * x x x x
     * x 0 0 x
     * x 1 x x
     * x x x 1
     */
    let map: TileMap = HashMap::from([([1, 1], 0), ([1, 2], 0), ([2, 1], 1), ([3, 3], 1)]);
    let map_size = [4, 4];

    let reverse = |nodes: Option<Nodes>| match nodes {
        Some(arr) => {
            let mut reversed = arr.clone();
            reversed.reverse();
            Some(reversed)
        }
        None => None,
    };

    let (coord1a, coord1b) = ([1, 1], [1, 2]);
    let expected1 = Some(vec![coord1a, coord1b]);
    assert_eq!(
        try_get_node_connection(&coord1a, &coord1b, &map, &map_size),
        expected1
    );
    assert_eq!(
        try_get_node_connection(&coord1b, &coord1a, &map, &map_size),
        reverse(expected1)
    );

    let (coord2a, coord2b) = ([0, 0], [1, 0]);
    let expected2 = Some(vec![coord2a, coord2b]);
    assert_eq!(
        try_get_node_connection(&coord2a, &coord2b, &map, &map_size),
        expected2
    );
    assert_eq!(
        try_get_node_connection(&coord2b, &coord2a, &map, &map_size),
        reverse(expected2)
    );

    let (coord3a, coord3b) = ([0, 0], [0, 3]);
    let expected3 = Some(vec![coord3a, coord3b]);
    assert_eq!(
        try_get_node_connection(&coord3a, &coord3b, &map, &map_size),
        expected3
    );
    assert_eq!(
        try_get_node_connection(&coord3b, &coord3a, &map, &map_size),
        reverse(expected3)
    );

    let (coord4a, coord4b) = ([0, 0], [3, 0]);
    let expected4 = Some(vec![coord4a, coord4b]);
    assert_eq!(
        try_get_node_connection(&coord4a, &coord4b, &map, &map_size),
        expected4
    );
    assert_eq!(
        try_get_node_connection(&coord4b, &coord4a, &map, &map_size),
        reverse(expected4)
    );

    let (coord5a, coord5b) = ([2, 1], [3, 3]);
    let expected5a = Some(vec![coord5a, [3, 1], coord5b]);
    let expected5b = Some(vec![coord5b, [2, 3], coord5a]);
    assert_eq!(
        try_get_node_connection(&coord5a, &coord5b, &map, &map_size),
        expected5a
    );
    assert_eq!(
        try_get_node_connection(&coord5b, &coord5a, &map, &map_size),
        expected5b
    );

    let (coord6a, coord6b) = ([0, 3], [3, 0]);
    let expected6 = Some(vec![coord6a, [0, 0], coord6b]);
    assert_eq!(
        try_get_node_connection(&coord6a, &coord6b, &map, &map_size),
        expected6
    );
    assert_eq!(
        try_get_node_connection(&coord6b, &coord6a, &map, &map_size),
        reverse(expected6)
    );

    let (coord7a, coord7b) = ([2, 3], [3, 1]);
    let expected7 = Some(vec![coord7a, [2, 2], [3, 2], coord7b]);
    assert_eq!(
        try_get_node_connection(&coord7a, &coord7b, &map, &map_size),
        expected7
    );
    assert_eq!(
        try_get_node_connection(&coord7b, &coord7a, &map, &map_size),
        reverse(expected7)
    );

    let (coord8a, coord8b) = ([1, 0], [1, 3]);
    let expected8 = Some(vec![coord8a, [0, 0], [0, 3], coord8b]);
    assert_eq!(
        try_get_node_connection(&coord8a, &coord8b, &map, &map_size),
        expected8
    );
    assert_eq!(
        try_get_node_connection(&coord8b, &coord8a, &map, &map_size),
        reverse(expected8)
    );

    let (coord9a, coord9b) = ([1, 0], [2, 2]);
    let expected9 = Some(vec![coord9a, [3, 0], [3, 2], coord9b]);
    assert_eq!(
        try_get_node_connection(&coord9a, &coord9b, &map, &map_size),
        expected9
    );
    assert_eq!(
        try_get_node_connection(&coord9b, &coord9a, &map, &map_size),
        reverse(expected9)
    );

    let (coord10a, coord10b) = ([0, 1], [3, 2]);
    let expected10 = Some(vec![coord10a, [0, 0], [3, 0], coord10b]);
    assert_eq!(
        try_get_node_connection(&coord10a, &coord10b, &map, &map_size),
        expected10
    );
    assert_eq!(
        try_get_node_connection(&coord10b, &coord10a, &map, &map_size),
        reverse(expected10)
    );

    let (coord11a, coord11b) = ([0, 1], [2, 2]);
    let expected11 = Some(vec![coord11a, [0, 3], [2, 3], coord11b]);
    assert_eq!(
        try_get_node_connection(&coord11a, &coord11b, &map, &map_size),
        expected11
    );
    assert_eq!(
        try_get_node_connection(&coord11b, &coord11a, &map, &map_size),
        reverse(expected11)
    );

    let (coord12a, coord12b) = ([1, 3], [3, 2]);
    let expected12 = Some(vec![coord12a, [2, 3], [2, 2], coord12b]);
    assert_eq!(
        try_get_node_connection(&coord12a, &coord12b, &map, &map_size),
        expected12
    );
    assert_eq!(
        try_get_node_connection(&coord12b, &coord12a, &map, &map_size),
        reverse(expected12)
    );
}
