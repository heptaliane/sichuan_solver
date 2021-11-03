use super::components::{
    Coord, CoordElement, Grid, Nodes, Tile, TileMap, TypedCoord, TypedCoordElement,
};
use std::cmp::{max, min};
use std::collections::hash_map::HashMap;

fn move_coord(map: &TileMap, coord: &Coord, delta: &TypedCoord) -> Option<Coord> {
    let moved: TypedCoord = [
        coord[0] as TypedCoordElement + delta[0],
        coord[1] as TypedCoordElement + delta[1],
    ];
    let shape = map.shape();

    match moved {
        [x, y] if x < 0 || y < 0 => None,
        [x, y] if x >= (shape[0] as i32) || y >= (shape[1] as i32) => None,
        _ => match [moved[0] as CoordElement, moved[1] as CoordElement] {
            trusted_moved if map[trusted_moved] == None => Some(trusted_moved),
            _ => None,
        },
    }
}

fn get_grid(map: &TileMap, &coord: &Coord, delta: &TypedCoord) -> Option<Grid> {
    let mut cursor = coord.clone();
    let mut next_cursor = move_coord(map, &coord, delta);
    loop {
        match next_cursor {
            None => break,
            Some(crd) if map[crd] != None => break,
            _ => {
                cursor = next_cursor.unwrap();
                next_cursor = move_coord(map, &cursor, delta)
            }
        }
    }
    Some([coord, cursor])
}

fn get_intersection(&grid1: &Grid, &grid2: &Grid) -> Option<Coord> {
    match (grid1, grid2) {
        ([[x11, y11], [x12, y12]], [[x21, y21], [x22, y22]]) if x11 == x12 && y21 == y22 => {
            match (x11, y21) {
                (x, y)
                    if max(x21, x22) >= x
                        && min(x21, x22) <= x
                        && max(y11, y12) >= y
                        && min(y11, y12) <= y =>
                {
                    Some([x, y])
                }
                _ => None,
            }
        }
        ([[x11, y11], [x12, y12]], [[x21, y21], [x22, y22]]) if y11 == y12 && x21 == x22 => {
            match (y11, x21) {
                (y, x)
                    if max(y21, y22) >= y
                        && min(y21, y22) <= y
                        && max(x11, x12) >= x
                        && min(x11, x12) <= x =>
                {
                    Some([x, y])
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn explore_x_connection(
    map: &TileMap,
    &xrange: &[CoordElement; 2],
    &yrange: &[CoordElement; 2],
) -> Option<Grid> {
    let (ymax, ymin) = match yrange {
        [y1, y2] if y1 > y2 => (y1, y2),
        [y1, y2] => (y2, y1),
    };

    for x in xrange[0]..=xrange[1] {
        let grid = get_grid(map, &[x, ymin], &[0, 1]);
        match grid {
            Some([_, [_, y]]) if y >= ymax => return Some([[x, ymin], [x, ymax]]),
            _ => (),
        };
    }
    None
}

fn explore_y_connection(
    map: &TileMap,
    &yrange: &[CoordElement; 2],
    &xrange: &[CoordElement; 2],
) -> Option<Grid> {
    let (xmax, xmin) = match xrange {
        [x1, x2] if x1 > x2 => (x1, x2),
        [x1, x2] => (x2, x1),
    };

    for y in yrange[0]..=yrange[1] {
        let grid = get_grid(map, &[xmin, y], &[1, 0]);
        match grid {
            Some([_, [x, _]]) if x >= xmax => return Some([[xmin, y], [xmax, y]]),
            _ => (),
        };
    }
    None
}

fn is_tile_touched(&coord1: &Coord, &coord2: &Coord) -> bool {
    (coord1[0] == coord2[0] && (coord1[1] == coord2[1] + 1 || coord1[1] + 1 == coord2[1]))
        || (coord1[1] == coord2[1] && (coord1[0] == coord2[0] + 1 || coord1[0] + 1 == coord2[0]))
}

fn is_connected_with_single_line(&grid1: &Grid, &grid2: &Grid) -> bool {
    let [[x11, y11], [x12, y12]] = grid1;
    let [[x21, y21], [x22, y22]] = grid2;

    (x11 == x12
        && x12 == x21
        && x21 == x22
        && max(y11, y12) >= min(y21, y22)
        && max(y21, y22) >= min(y11, y12))
        || (y11 == y12
            && y12 == y21
            && y21 == y22
            && max(x11, x12) >= min(x21, x22)
            && max(x21, x22) >= min(x11, x12))
}

fn get_double_line_connection(grid1: &Grid, grid2: &Grid) -> Option<Nodes> {
    let intersection = get_intersection(grid1, grid2);
    match intersection {
        None => None,
        _ => Some([Some(grid1[0]), intersection, Some(grid2[0]), None]),
    }
}

fn get_triple_line_connection(map: &TileMap, &grid1: &Grid, &grid2: &Grid) -> Option<Nodes> {
    if grid1[0][0] == grid1[1][0] && grid2[0][0] == grid2[1][0] {
        let grid3 = match (grid1, grid2) {
            ([[x1, y11], [_, y12]], [[x2, y21], [_, y22]]) if y11 >= y21 && y12 <= y22 => {
                explore_y_connection(map, &[y12, y22], &[x1, x2])
            }
            ([[x1, y11], [_, y12]], [[x2, y21], [_, y22]]) if y11 <= y21 && y12 >= y22 => {
                explore_y_connection(map, &[y22, y12], &[x1, x2])
            }
            _ => None,
        };
        return match grid3 {
            Some(grid) if grid[0][0] == grid1[0][0] => {
                Some([Some(grid1[0]), Some(grid[0]), Some(grid[1]), Some(grid2[0])])
            }
            Some(grid) if grid[0][0] == grid2[0][0] => {
                Some([Some(grid2[0]), Some(grid[0]), Some(grid[1]), Some(grid1[0])])
            }
            _ => None,
        };
    }

    if grid1[0][1] == grid1[1][1] && grid2[0][1] == grid2[1][1] {
        let grid3 = match (grid1, grid2) {
            ([[x11, y1], [x12, _]], [[x21, y2], [x22, _]]) if x11 >= x21 && x12 <= x22 => {
                println!("[{}, {}] [{}, {}]", x12, x22, y1, y2);
                explore_x_connection(map, &[x12, x22], &[y1, y2])
            }
            ([[x11, y1], [x12, _]], [[x21, y2], [x22, _]]) if x11 <= x21 && x12 >= x22 => {
                explore_x_connection(map, &[x22, x12], &[y1, y2])
            }
            _ => None,
        };
        return match grid3 {
            Some(grid) if grid[0][1] == grid1[0][1] => {
                Some([Some(grid1[0]), Some(grid[0]), Some(grid[1]), Some(grid2[0])])
            }
            Some(grid) if grid[0][1] == grid2[0][1] => {
                Some([Some(grid2[0]), Some(grid[0]), Some(grid[1]), Some(grid1[0])])
            }
            _ => None,
        };
    }
    None
}

fn get_grid_pair(
    map: &TileMap,
    coord1: &Coord,
    coord2: &Coord,
    directions: Option<[TypedCoord; 2]>,
) -> Option<[Grid; 2]> {
    match directions {
        Some([dir1, dir2]) => {
            let grid1 = get_grid(map, coord1, &dir1);
            let grid2 = get_grid(map, coord2, &dir2);
            match (grid1, grid2) {
                (Some(g1), Some(g2)) => Some([g1, g2]),
                _ => None,
            }
        }
        None => None,
    }
}

fn get_faced_x_grid_pair(map: &TileMap, coord1: &Coord, coord2: &Coord) -> Option<[Grid; 2]> {
    let directions = match (coord1, coord2) {
        ([x1, _], [x2, _]) if x1 > x2 => Some([[-1, 0], [1, 0]]),
        ([x1, _], [x2, _]) if x1 < x2 => Some([[1, 0], [-1, 0]]),
        _ => None,
    };

    get_grid_pair(map, coord1, coord2, directions)
}

fn get_faced_y_grid_pair(map: &TileMap, coord1: &Coord, coord2: &Coord) -> Option<[Grid; 2]> {
    let directions = match (coord1, coord2) {
        ([_, y1], [_, y2]) if y1 > y2 => Some([[0, -1], [0, 1]]),
        ([_, y1], [_, y2]) if y1 < y2 => Some([[0, 1], [0, -1]]),
        _ => None,
    };

    get_grid_pair(map, coord1, coord2, directions)
}

fn get_l_grid_pair(map: &TileMap, coord1: &Coord, coord2: &Coord) -> [Option<[Grid; 2]>; 2] {
    let xs = match (coord1, coord2) {
        ([x1, _], [x2, _]) if x1 > x2 => Some([-1, 1]),
        ([x1, _], [x2, _]) if x1 < x2 => Some([1, -1]),
        _ => None,
    };
    let ys = match (coord1, coord2) {
        ([_, y1], [_, y2]) if y1 > y2 => Some([-1, 1]),
        ([_, y1], [_, y2]) if y1 < y2 => Some([1, -1]),
        _ => None,
    };
    let directions = match (xs, ys) {
        (Some(x), Some(y)) => [Some([[x[0], 0], [0, y[1]]]), Some([[0, y[0]], [x[1], 0]])],
        _ => [None, None],
    };
    directions.map(|dirs| get_grid_pair(map, coord1, coord2, dirs))
}

fn get_parallel_grid_pair(map: &TileMap, coord1: &Coord, coord2: &Coord) -> [Option<[Grid; 2]>; 4] {
    [[1, 0], [-1, 0], [0, 1], [0, -1]]
        .map(|dir| get_grid_pair(map, coord1, coord2, Some([dir, dir])))
}

fn find_connection(map: &TileMap, coord1: &Coord, coord2: &Coord) -> Option<Nodes> {
    if is_tile_touched(coord1, coord2) {
        return Some([Some(*coord1), Some(*coord2), None, None]);
    }

    let grids1 = get_faced_x_grid_pair(map, coord1, coord2);
    let grids2 = get_faced_y_grid_pair(map, coord1, coord2);
    for grids in [grids1, grids2] {
        if let Some(gs) = grids {
            if is_connected_with_single_line(&gs[0], &gs[1]) {
                return Some([Some(*coord1), Some(*coord2), None, None]);
            }
        }
    }

    let [grids3, grids4] = get_l_grid_pair(map, coord1, coord2);
    for grids in [grids3, grids4] {
        match grids {
            Some([g1, g2]) => match get_double_line_connection(&g1, &g2) {
                None => (),
                path => return path,
            },
            _ => (),
        };
    }

    let [grids5, grids6, grids7, grids8] = get_parallel_grid_pair(map, coord1, coord2);
    for grids in [grids1, grids2, grids5, grids6, grids7, grids8] {
        match grids {
            Some([g1, g2]) => match get_triple_line_connection(map, &g1, &g2) {
                None => (),
                path => return path,
            },
            _ => (),
        }
    }

    None
}

#[test]
fn test_move_coord() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [None, Some(0), None, Some(0)],
        [Some(0), None, None, None],
        [None, None, Some(0), None],
    ]);
    /* tile map:
     * x 0 x 0
     * 0 x x x
     * x x 0 x
     */

    assert_eq!(move_coord(&map, &[1, 1], &[1, 0]), Some([2, 1]));
    assert_eq!(move_coord(&map, &[1, 1], &[0, 1]), Some([1, 2]));
    assert_eq!(move_coord(&map, &[2, 1], &[-1, 0]), Some([1, 1]));
    assert_eq!(move_coord(&map, &[2, 1], &[0, -1]), Some([2, 0]));
    assert_eq!(move_coord(&map, &[0, 0], &[1, 0]), None);
    assert_eq!(move_coord(&map, &[0, 0], &[0, 1]), None);
    assert_eq!(move_coord(&map, &[1, 1], &[-1, 0]), None);
    assert_eq!(move_coord(&map, &[1, 1], &[0, -1]), None);
    assert_eq!(move_coord(&map, &[0, 0], &[-1, 0]), None);
    assert_eq!(move_coord(&map, &[0, 0], &[0, -1]), None);
    assert_eq!(move_coord(&map, &[2, 3], &[1, 0]), None);
    assert_eq!(move_coord(&map, &[2, 3], &[0, 1]), None);
}

#[test]
fn test_get_grid() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [None, Some(0), None, Some(0)],
        [Some(0), None, None, None],
        [None, None, None, Some(0)],
    ]);
    /* tile map:
     * x 0 x 0
     * 0 x x x
     * x x x 0
     */

    assert_eq!(get_grid(&map, &[0, 1], &[0, 1]), Some([[0, 1], [0, 2]]));
    assert_eq!(get_grid(&map, &[0, 3], &[0, -1]), Some([[0, 3], [0, 2]]));
    assert_eq!(get_grid(&map, &[0, 3], &[1, 0]), Some([[0, 3], [1, 3]]));
    assert_eq!(get_grid(&map, &[2, 3], &[-1, 0]), Some([[2, 3], [1, 3]]));
    assert_eq!(get_grid(&map, &[1, 0], &[0, 1]), Some([[1, 0], [1, 3]]));
    assert_eq!(get_grid(&map, &[2, 3], &[0, -1]), Some([[2, 3], [2, 0]]));
    assert_eq!(get_grid(&map, &[1, 0], &[1, 0]), Some([[1, 0], [2, 0]]));
    assert_eq!(get_grid(&map, &[1, 0], &[-1, 0]), Some([[1, 0], [0, 0]]));
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
fn test_explore_x_connection() {
    use ndarray::arr2;
    let map1: TileMap = arr2(&[
        [None, None, None],
        [None, Some(0), None],
        [None, None, None],
        [None, Some(0), None],
    ]);
    /* tile map:
     * | x x
     * | 0 |
     * | x |
     * | 0 |
     */
    assert_eq!(
        explore_x_connection(&map1, &[1, 3], &[0, 2]),
        Some([[2, 0], [2, 2]])
    );
    assert_eq!(explore_x_connection(&map1, &[1, 1], &[0, 2]), None);

    let map2: TileMap = arr2(&[
        [None, None, None],
        [None, Some(0), None],
        [None, Some(0), None],
        [None, Some(0), None],
    ]);
    assert_eq!(explore_x_connection(&map2, &[1, 3], &[0, 2]), None);
}

#[test]
fn test_explore_y_connection() {
    use ndarray::arr2;
    let map1: TileMap = arr2(&[
        [None, None, None, None],
        [None, Some(0), None, Some(0)],
        [None, None, None, None],
    ]);
    /* tile map:
     * -------
     * x 0 x 0
     * x -----
     */
    assert_eq!(
        explore_y_connection(&map1, &[1, 3], &[0, 2]),
        Some([[0, 2], [2, 2]])
    );
    assert_eq!(explore_y_connection(&map1, &[1, 1], &[0, 2]), None);

    let map2: TileMap = arr2(&[
        [None, None, None, None],
        [None, Some(0), Some(0), Some(0)],
        [None, None, None, None],
    ]);
    assert_eq!(explore_y_connection(&map2, &[1, 3], &[0, 2]), None);
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
fn test_get_double_line_connection() {
    let to_path =
        |&c1: &Coord, &c2: &Coord, &c3: &Coord| Some([Some(c1), Some(c2), Some(c3), None]);

    let test_connection = |&ga: &Grid, &gb: &Grid, &intersection: &Option<Coord>| {
        let actual = get_double_line_connection(&ga, &gb);
        match intersection {
            Some(coord) => assert_eq!(actual, to_path(&ga[0], &coord, &gb[0])),
            None => assert_eq!(actual, None),
        }
    };

    let check_all = |grid1: Grid, grid2: Grid, expected: Option<Coord>| {
        for [g1, g2] in [[grid1, grid2], [grid2, grid1]] {
            for ga in [g1, [g1[1], g1[0]]] {
                for gb in [g2, [g2[1], g2[0]]] {
                    test_connection(&ga, &gb, &expected);
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
fn test_get_triple_line_connection() {
    let to_path = |coords: Option<[Coord; 4]>| -> Option<Nodes> {
        match coords {
            Some([c1, c2, c3, c4]) => Some([Some(c1), Some(c2), Some(c3), Some(c4)]),
            None => None,
        }
    };

    let check_all = |map: &TileMap, grid1: Grid, grid2: Grid, expected: Option<Nodes>| {
        for (g1, g2) in [(grid1, grid2), (grid2, grid1)] {
            let actual = get_triple_line_connection(map, &g1, &g2);
            assert_eq!(actual, expected);
        }
    };

    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [None, None, None, None],
        [None, None, Some(0), None],
        [None, None, None, None],
        [None, None, Some(0), None],
    ]);
    /*
     * x x x x
     * x x 0 x
     * x x x x
     * x x 0 x
     */
    check_all(
        &map,
        [[0, 0], [0, 1]],
        [[1, 2], [1, 1]],
        to_path(Some([[0, 0], [0, 1], [1, 1], [1, 2]])),
    );
    check_all(
        &map,
        [[0, 2], [0, 1]],
        [[1, 0], [1, 1]],
        to_path(Some([[0, 2], [0, 1], [1, 1], [1, 0]])),
    );
    check_all(
        &map,
        [[0, 0], [1, 0]],
        [[2, 1], [1, 1]],
        to_path(Some([[0, 0], [1, 0], [1, 1], [2, 1]])),
    );
    check_all(
        &map,
        [[0, 2], [0, 1]],
        [[1, 0], [1, 1]],
        to_path(Some([[0, 2], [0, 1], [1, 1], [1, 0]])),
    );
    check_all(
        &map,
        [[0, 0], [0, 2]],
        [[2, 3], [2, 1]],
        to_path(Some([[0, 0], [0, 1], [2, 1], [2, 3]])),
    );
    check_all(
        &map,
        [[0, 3], [0, 1]],
        [[1, 0], [1, 2]],
        to_path(Some([[0, 3], [0, 1], [1, 1], [1, 0]])),
    );
    check_all(
        &map,
        [[0, 0], [2, 0]],
        [[3, 1], [1, 1]],
        to_path(Some([[0, 0], [1, 0], [1, 1], [3, 1]])),
    );
    check_all(
        &map,
        [[3, 0], [1, 0]],
        [[0, 1], [2, 1]],
        to_path(Some([[3, 0], [1, 0], [1, 1], [0, 1]])),
    );
    check_all(&map, [[0, 1], [0, 2]], [[2, 3], [2, 2]], to_path(None));
    check_all(&map, [[0, 3], [0, 2]], [[2, 1], [2, 2]], to_path(None));
    check_all(&map, [[0, 1], [1, 1]], [[2, 3], [1, 3]], to_path(None));
    check_all(&map, [[2, 1], [1, 1]], [[0, 3], [1, 3]], to_path(None));
    check_all(
        &map,
        [[0, 0], [1, 0]],
        [[0, 1], [1, 1]],
        to_path(Some([[0, 0], [1, 0], [1, 1], [0, 1]])),
    );
    check_all(
        &map,
        [[1, 0], [0, 0]],
        [[1, 1], [0, 1]],
        to_path(Some([[1, 0], [0, 0], [0, 1], [1, 1]])),
    );
    check_all(
        &map,
        [[0, 0], [0, 1]],
        [[1, 0], [1, 1]],
        to_path(Some([[0, 0], [0, 1], [1, 1], [1, 0]])),
    );
    check_all(
        &map,
        [[0, 1], [0, 0]],
        [[1, 1], [1, 0]],
        to_path(Some([[0, 1], [0, 0], [1, 0], [1, 1]])),
    );
    check_all(
        &map,
        [[0, 0], [3, 0]],
        [[0, 3], [3, 3]],
        to_path(Some([[0, 0], [2, 0], [2, 3], [0, 3]])),
    );
    check_all(
        &map,
        [[3, 0], [0, 0]],
        [[3, 3], [0, 3]],
        to_path(Some([[3, 0], [0, 0], [0, 3], [3, 3]])),
    );
    check_all(
        &map,
        [[0, 0], [0, 3]],
        [[2, 0], [2, 3]],
        to_path(Some([[0, 0], [0, 1], [2, 1], [2, 0]])),
    );
    check_all(
        &map,
        [[0, 3], [0, 0]],
        [[2, 3], [2, 0]],
        to_path(Some([[0, 3], [0, 0], [2, 0], [2, 3]])),
    );
}
