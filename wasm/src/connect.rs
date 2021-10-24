use super::components::{
    Coord, CoordElement, Grid, Path, Tile, TileMap, TypedCoord, TypedCoordElement,
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
        _ => Some([moved[0] as CoordElement, moved[1] as CoordElement]),
    }
}

fn get_grid(map: &TileMap, &coord: &Coord, delta: &TypedCoord) -> Option<Grid> {
    if map[coord] == None {
        return None;
    }

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

fn get_grid_coordinate(grid: &Grid) -> Vec<Coord> {
    match grid {
        [crd1, crd2] if crd1[0] == crd2[0] => (crd1[1]..=crd2[1])
            .map(|y| [crd1[0], y])
            .collect::<Vec<Coord>>(),
        [crd1, crd2] if crd1[1] == crd2[1] => (crd1[0]..=crd2[0])
            .map(|x| [x, crd1[1]])
            .collect::<Vec<Coord>>(),
        _ => vec![],
    }
}

fn get_intersection(grid1: &Grid, grid2: &Grid) -> Option<Coord> {
    let coords1 = get_grid_coordinate(grid1);
    let coords2 = get_grid_coordinate(grid2);
    let mut lut = HashMap::<Coord, usize>::new();

    for coord in [coords1, coords2].concat() {
        match lut.get(&coord) {
            Some(cnt) => lut.insert(coord, cnt + 1),
            None => lut.insert(coord, 0),
        };
    }

    let intersections = lut
        .iter()
        .filter(|(_, &cnt)| cnt > 1)
        .map(|(&crd, _)| crd)
        .collect::<Vec<Coord>>();
    match intersections.len() {
        1 => Some(intersections[0]),
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

    for x in xrange[0]..xrange[1] {
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
    (coord1[0] == coord2[0] && (coord1[1] == coord2[1] + 1) || (coord1[1] + 1 == coord2[1]))
        || (coord1[1] == coord2[1] && (coord1[0] == coord2[0] + 1) || (coord1[0] + 1 == coord2[0]))
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

fn get_double_line_connection(grid1: &Grid, grid2: &Grid) -> Option<Path> {
    let intersection = get_intersection(grid1, grid2);
    match intersection {
        None => None,
        _ => Some([Some(grid1[0]), intersection, Some(grid2[0]), None]),
    }
}

fn get_triple_line_connection(map: &TileMap, &grid1: &Grid, &grid2: &Grid) -> Option<Path> {
    let coords = get_grid_coordinate(&grid1);

    if grid1[0][0] == grid1[1][0] && grid2[0][0] == grid2[1][0] {
        let grid3 = match (grid1, grid2) {
            ([[x1, y11], [_, y12]], [[x2, y21], [_, y22]])
                if (y11 > y21 && y12 < y22) || (y11 < y21 && y12 > y22) =>
            {
                explore_y_connection(map, &[y12, y22], &[x1, x2])
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
            ([[x11, y1], [x12, _]], [[x21, y2], [x22, _]])
                if (x11 > x21 && x12 < x22) || (x11 < x21 && x12 > x22) =>
            {
                explore_x_connection(map, &[x12, x22], &[y1, y2])
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

fn find_connection(map: &TileMap, coord1: &Coord, coord2: &Coord) -> Option<Path> {
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
