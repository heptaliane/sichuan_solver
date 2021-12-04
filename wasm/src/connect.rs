use super::components::{
    Coord, CoordElement, Grid, Nodes, Tile, TileMap, TypedCoord, TypedCoordElement,
};
use std::cmp::{max, min};

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

fn get_overlapped_x_range(&grid1: &Grid, &grid2: &Grid) -> Option<[CoordElement; 2]> {
    let sort_x = |x1: CoordElement, x2: CoordElement| if x1 > x2 { (x1, x2) } else { (x2, x1) };
    match (grid1, grid2) {
        ([[_, y11], [_, y12]], [[_, y21], [_, y22]]) if y11 != y12 || y21 != y22 => None,
        ([[x11, _], [x12, _]], [[x21, _], [x22, _]]) => {
            let (xmax1, xmin1) = sort_x(x11, x12);
            let (xmax2, xmin2) = sort_x(x21, x22);
            match (min(xmax1, xmax2), max(xmin1, xmin2)) {
                (xmax, xmin) if xmax >= xmin => Some([xmin, xmax]),
                _ => None,
            }
        }
    }
}

fn get_overlapped_y_range(&grid1: &Grid, &grid2: &Grid) -> Option<[CoordElement; 2]> {
    let sort_y = |y1: CoordElement, y2: CoordElement| if y1 > y2 { (y1, y2) } else { (y2, y1) };
    match (grid1, grid2) {
        ([[x11, _], [x12, _]], [[x21, _], [x22, _]]) if x11 != x12 || x21 != x22 => None,
        ([[_, y11], [_, y12]], [[_, y21], [_, y22]]) => {
            let (ymax1, ymin1) = sort_y(y11, y12);
            let (ymax2, ymin2) = sort_y(y21, y22);
            match (min(ymax1, ymax2), max(ymin1, ymin2)) {
                (ymax, ymin) if ymax >= ymin => Some([ymin, ymax]),
                _ => None,
            }
        }
    }
}

fn get_triple_line_connection(map: &TileMap, &grid1: &Grid, &grid2: &Grid) -> Option<Nodes> {
    if let Some(xrange) = get_overlapped_x_range(&grid1, &grid2) {
        let grid3 = explore_x_connection(map, &xrange, &[grid1[0][1], grid2[0][1]]);
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

    if let Some(yrange) = get_overlapped_y_range(&grid1, &grid2) {
        let grid3 = explore_y_connection(map, &yrange, &[grid1[0][0], grid2[0][0]]);
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
fn test_get_overlapped_x_range() {
    let check_all = |grid1: Grid, grid2: Grid, expected: Option<[CoordElement; 2]>| {
        for (g1, g2) in [(grid1, grid2), (grid2, grid1)] {
            for ga in [g1, [g1[1], g1[0]]] {
                for gb in [g2, [g2[1], g2[0]]] {
                    assert_eq!(get_overlapped_x_range(&ga, &gb), expected);
                }
            }
        }
    };

    check_all([[0, 0], [1, 0]], [[1, 0], [2, 0]], Some([1, 1]));
    check_all([[0, 0], [2, 0]], [[1, 0], [2, 0]], Some([1, 2]));
    check_all([[0, 0], [2, 0]], [[1, 0], [3, 0]], Some([1, 2]));
    check_all([[0, 0], [1, 0]], [[2, 0], [3, 0]], None);
    check_all([[0, 0], [0, 1]], [[1, 0], [2, 0]], None);
    check_all([[0, 0], [0, 1]], [[0, 1], [0, 2]], None);
    check_all([[0, 0], [0, 2]], [[0, 1], [0, 2]], None);
    check_all([[0, 0], [0, 2]], [[0, 1], [0, 3]], None);
    check_all([[0, 0], [0, 1]], [[0, 2], [0, 3]], None);
}

#[test]
fn test_get_overlapped_y_range() {
    let check_all = |grid1: Grid, grid2: Grid, expected: Option<[CoordElement; 2]>| {
        for (g1, g2) in [(grid1, grid2), (grid2, grid1)] {
            for ga in [g1, [g1[1], g1[0]]] {
                for gb in [g2, [g2[1], g2[0]]] {
                    assert_eq!(get_overlapped_y_range(&ga, &gb), expected);
                }
            }
        }
    };

    check_all([[0, 0], [1, 0]], [[1, 0], [2, 0]], None);
    check_all([[0, 0], [2, 0]], [[1, 0], [2, 0]], None);
    check_all([[0, 0], [2, 0]], [[1, 0], [3, 0]], None);
    check_all([[0, 0], [1, 0]], [[2, 0], [3, 0]], None);
    check_all([[0, 0], [0, 1]], [[1, 0], [2, 0]], None);
    check_all([[0, 0], [0, 1]], [[0, 1], [0, 2]], Some([1, 1]));
    check_all([[0, 0], [0, 2]], [[0, 1], [0, 2]], Some([1, 2]));
    check_all([[0, 0], [0, 2]], [[0, 1], [0, 3]], Some([1, 2]));
    check_all([[0, 0], [0, 1]], [[0, 2], [0, 3]], None);
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
        [[1, 1], [2, 1]],
        [[1, 3], [2, 3]],
        to_path(Some([[1, 1], [2, 1], [2, 3], [1, 3]])),
    );
    check_all(
        &map,
        [[1, 1], [3, 1]],
        [[0, 3], [3, 3]],
        to_path(Some([[1, 1], [2, 1], [2, 3], [0, 3]])),
    );
    check_all(&map, [[1, 0], [1, 1]], [[2, 3], [3, 3]], None);
    check_all(&map, [[0, 1], [1, 1]], [[1, 3], [3, 3]], None);
    check_all(
        &map,
        [[3, 1], [2, 1]],
        [[3, 3], [2, 3]],
        to_path(Some([[3, 1], [2, 1], [2, 3], [3, 3]])),
    );
    check_all(
        &map,
        [[3, 1], [1, 1]],
        [[3, 3], [0, 3]],
        to_path(Some([[3, 1], [2, 1], [2, 3], [3, 3]])),
    );
    check_all(&map, [[3, 1], [2, 1]], [[1, 3], [0, 3]], None);
    check_all(&map, [[3, 1], [1, 1]], [[1, 3], [0, 3]], None);
    check_all(
        &map,
        [[0, 2], [0, 3]],
        [[2, 2], [2, 3]],
        to_path(Some([[0, 2], [0, 3], [2, 3], [2, 2]])),
    );
    check_all(
        &map,
        [[0, 1], [0, 3]],
        [[2, 2], [2, 3]],
        to_path(Some([[0, 1], [0, 3], [2, 3], [2, 2]])),
    );
    check_all(&map, [[0, 0], [0, 1]], [[2, 2], [2, 3]], None);
    check_all(&map, [[0, 0], [0, 2]], [[2, 2], [2, 3]], None);
    check_all(
        &map,
        [[0, 2], [0, 1]],
        [[2, 2], [2, 1]],
        to_path(Some([[0, 2], [0, 1], [2, 1], [2, 2]])),
    );
    check_all(
        &map,
        [[0, 3], [0, 0]],
        [[2, 2], [2, 1]],
        to_path(Some([[0, 3], [0, 1], [2, 1], [2, 2]])),
    );
    check_all(&map, [[0, 3], [0, 2]], [[2, 1], [2, 0]], None);
    check_all(&map, [[0, 3], [0, 2]], [[2, 2], [2, 0]], None);
}

#[test]
fn test_get_faced_x_grid_pair() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [None, Some(0), None, Some(1)],
        [None, None, None, None],
        [None, Some(0), Some(1), None],
        [None, None, None, None],
    ]);
    /*
     * x 0 x 1
     * x x x x
     * x 0 1 x
     * x x x x
     */
    let check_all = |coord1: Coord, coord2: Coord, expected: Option<[Grid; 2]>| {
        if let Some(grids) = expected {
            for (c1, c2, ex) in [
                (coord1, coord2, grids),
                (coord2, coord1, [grids[1], grids[0]]),
            ] {
                assert_eq!(get_faced_x_grid_pair(&map, &c1, &c2), Some(ex));
            }
        }
    };
    check_all([0, 0], [3, 0], Some([[[0, 0], [3, 0]], [[3, 0], [0, 0]]]));
    check_all([0, 0], [3, 3], Some([[[0, 0], [3, 0]], [[3, 3], [1, 3]]]));
    check_all([0, 0], [1, 2], Some([[[0, 0], [3, 0]], [[1, 2], [0, 2]]]));
    check_all([0, 0], [0, 2], None);
    check_all([0, 0], [3, 2], None);
}

#[test]
fn test_get_faced_y_grid_pair() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [None, Some(0), None, Some(1)],
        [None, None, None, None],
        [None, Some(0), Some(1), None],
        [None, None, None, None],
    ]);
    /*
     * x 0 x 1
     * x x x x
     * x 0 1 x
     * x x x x
     */
    let check_all = |coord1: Coord, coord2: Coord, expected: Option<[Grid; 2]>| {
        if let Some(grids) = expected {
            for (c1, c2, ex) in [
                (coord1, coord2, grids),
                (coord2, coord1, [grids[1], grids[0]]),
            ] {
                assert_eq!(get_faced_y_grid_pair(&map, &c1, &c2), Some(ex));
            }
        }
    };
    check_all([1, 0], [1, 3], Some([[[1, 0], [1, 3]], [[1, 3], [1, 0]]]));
    check_all([1, 0], [3, 3], Some([[[1, 0], [1, 3]], [[3, 3], [3, 0]]]));
    check_all([1, 1], [3, 2], Some([[[1, 1], [1, 3]], [[3, 2], [3, 0]]]));
    check_all([0, 0], [1, 2], None);
    check_all([1, 2], [3, 2], None);
}

#[test]
fn test_get_l_grid_pair() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [None, None, Some(0), None],
        [None, None, None, None],
        [None, Some(0), None, None],
        [None, None, None, None],
    ]);
    /*
     * x x 0 x
     * x x x x
     * x 0 x x
     * x x x x
     */
    let check_all = |coord1: Coord, coord2: Coord, expected: [Option<[Grid; 2]>; 2]| {
        let actual1 = get_l_grid_pair(&map, &coord1, &coord2);
        let actual2 = get_l_grid_pair(&map, &coord2, &coord1);
        let expected2 = match expected {
            [Some([g11, g21]), Some([g12, g22])] => [Some([g22, g12]), Some([g21, g11])],
            _ => [None, None],
        };
        assert_eq!(actual1, expected);
        assert_eq!(actual2, expected2);
    };

    check_all(
        [1, 0],
        [3, 2],
        [
            Some([[[1, 0], [3, 0]], [[3, 2], [3, 0]]]),
            Some([[[1, 0], [1, 3]], [[3, 2], [1, 2]]]),
        ],
    );
    check_all(
        [1, 2],
        [3, 0],
        [
            Some([[[1, 2], [3, 2]], [[3, 0], [3, 3]]]),
            Some([[[1, 2], [1, 0]], [[3, 0], [0, 0]]]),
        ],
    );
    check_all([0, 0], [1, 0], [None, None]);
    check_all([0, 0], [0, 1], [None, None]);
}

#[test]
fn test_get_parallel_grid_pair() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [None, None, None, None],
        [None, None, None, None],
        [None, None, None, None],
        [None, None, None, None],
    ]);

    let check_all = |coord1: Coord, coord2: Coord, expected: [Option<[Grid; 2]>; 4]| {
        let actual1 = get_parallel_grid_pair(&map, &coord1, &coord2);
        let actual2 = get_parallel_grid_pair(&map, &coord2, &coord1);
        let expected2 = expected.map(|g: Option<[Grid; 2]>| match g {
            Some([g1, g2]) => Some([g2, g1]),
            _ => None,
        });
        assert_eq!(actual1, expected);
        assert_eq!(actual2, expected2);
    };

    check_all(
        [1, 1],
        [2, 2],
        [
            Some([[[1, 1], [3, 1]], [[2, 2], [3, 2]]]),
            Some([[[1, 1], [0, 1]], [[2, 2], [0, 2]]]),
            Some([[[1, 1], [1, 3]], [[2, 2], [2, 3]]]),
            Some([[[1, 1], [1, 0]], [[2, 2], [2, 0]]]),
        ],
    );

    check_all(
        [1, 2],
        [2, 1],
        [
            Some([[[1, 2], [3, 2]], [[2, 1], [3, 1]]]),
            Some([[[1, 2], [0, 2]], [[2, 1], [0, 1]]]),
            Some([[[1, 2], [1, 3]], [[2, 1], [2, 3]]]),
            Some([[[1, 2], [1, 0]], [[2, 1], [2, 0]]]),
        ],
    );
}

#[test]
fn test_find_connection() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [None, None, None, None],
        [None, Some(0), Some(0), None],
        [None, Some(1), None, None],
        [None, None, None, Some(1)],
    ]);

    let check_all = |coord1: Coord, coord2: Coord, expected: Option<Nodes>| {
        let actual1 = find_connection(&map, &coord1, &coord2);
        let actual2 = find_connection(&map, &coord2, &coord1);
        let expected2 = match expected {
            Some([Some(c1), Some(c2), None, None]) => Some([Some(c2), Some(c1), None, None]),
            Some([Some(c1), Some(c2), Some(c3), None]) => {
                Some([Some(c3), Some(c2), Some(c1), None])
            }
            Some([Some(c1), Some(c2), Some(c3), Some(c4)]) => {
                Some([Some(c4), Some(c3), Some(c2), Some(c1)])
            }
            _ => None,
        };
        assert_eq!(actual1, expected);
        assert_eq!(actual2, expected2);
    };

    check_all(
        [1, 1],
        [1, 2],
        Some([Some([1, 1]), Some([1, 2]), None, None]),
    );
    check_all(
        [0, 0],
        [0, 3],
        Some([Some([0, 0]), Some([0, 3]), None, None]),
    );
    check_all(
        [0, 0],
        [3, 0],
        Some([Some([0, 0]), Some([3, 0]), None, None]),
    );
    assert_eq!(
        find_connection(&map, &[0, 0], &[1, 1]),
        Some([Some([0, 0]), Some([1, 0]), Some([1, 1]), None])
    );
    assert_eq!(
        find_connection(&map, &[1, 1], &[0, 0]),
        Some([Some([1, 1]), Some([0, 1]), Some([0, 0]), None])
    );
    assert_eq!(
        find_connection(&map, &[0, 1], &[1, 0]),
        Some([Some([0, 1]), Some([0, 0]), Some([1, 0]), None])
    );
    assert_eq!(
        find_connection(&map, &[1, 0], &[0, 1]),
        Some([Some([1, 0]), Some([0, 0]), Some([0, 1]), None])
    );
    assert_eq!(
        find_connection(&map, &[3, 0], &[2, 3]),
        Some([Some([2, 3]), Some([2, 2]), Some([3, 2]), Some([3, 0])]),
    );
    assert_eq!(
        find_connection(&map, &[2, 3], &[3, 0]),
        Some([Some([2, 3]), Some([2, 2]), Some([3, 2]), Some([3, 0])]),
    );
    assert_eq!(
        find_connection(&map, &[1, 0], &[1, 3]),
        Some([Some([1, 0]), Some([0, 0]), Some([0, 3]), Some([1, 3])]),
    );
    assert_eq!(
        find_connection(&map, &[1, 3], &[1, 0]),
        Some([Some([1, 0]), Some([0, 0]), Some([0, 3]), Some([1, 3])]),
    );
    assert_eq!(
        find_connection(&map, &[1, 3], &[1, 0]),
        Some([Some([1, 0]), Some([0, 0]), Some([0, 3]), Some([1, 3])]),
    );
    assert_eq!(
        find_connection(&map, &[1, 0], &[2, 2]),
        Some([Some([1, 0]), Some([3, 0]), Some([3, 2]), Some([2, 2])]),
    );
    assert_eq!(
        find_connection(&map, &[2, 2], &[1, 0]),
        Some([Some([1, 0]), Some([3, 0]), Some([3, 2]), Some([2, 2])]),
    );
    assert_eq!(
        find_connection(&map, &[0, 1], &[3, 2]),
        Some([Some([0, 1]), Some([0, 0]), Some([3, 0]), Some([3, 2])]),
    );
    assert_eq!(
        find_connection(&map, &[3, 2], &[0, 1]),
        Some([Some([0, 1]), Some([0, 0]), Some([3, 0]), Some([3, 2])]),
    );
    assert_eq!(
        find_connection(&map, &[0, 1], &[2, 2]),
        Some([Some([0, 1]), Some([0, 3]), Some([2, 3]), Some([2, 2])]),
    );
    assert_eq!(
        find_connection(&map, &[2, 2], &[0, 1]),
        Some([Some([0, 1]), Some([0, 3]), Some([2, 3]), Some([2, 2])]),
    );
    assert_eq!(
        find_connection(&map, &[1, 3], &[3, 2]),
        Some([Some([3, 2]), Some([2, 2]), Some([2, 3]), Some([1, 3])]),
    );
    assert_eq!(
        find_connection(&map, &[3, 2], &[1, 3]),
        Some([Some([3, 2]), Some([2, 2]), Some([2, 3]), Some([1, 3])]),
    );
}
