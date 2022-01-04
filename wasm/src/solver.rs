use super::components::{Coord, Nodes, Tile, TileMap};
use super::connect::find_connection;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;


#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct ConnectionInfo {
    tile: Tile,
    pair: [Coord; 2],
    nodes: Nodes,
}

type TileLut = HashMap<Tile, Vec<Coord>>;
type ConnectionLut = HashMap<Tile, Vec<ConnectionInfo>>;

fn create_tile_lookup_table(map: &TileMap) -> TileLut {
    let shape = map.shape();
    let mut lut: TileLut = HashMap::new();

    for x in 0..shape[0] {
        for y in 0..shape[1] {
            if let Some(tile) = map[[x, y]] {
                match lut.get_mut(&tile) {
                    Some(coords) => coords.push([x, y]),
                    _ => {
                        lut.insert(tile, vec![[x, y]]);
                    }
                };
            }
        }
    }
    lut
}

fn collect_connection_single_pair_tiles(map: &TileMap) -> Vec<ConnectionInfo> {
    let lut = create_tile_lookup_table(map);
    let mut connections: Vec<ConnectionInfo> = Vec::new();

    for (&tile, coords) in lut.iter().filter(|(_, coord)| coord.len() == 2) {
        match find_connection(map, &coords[0], &coords[1]) {
            Some(nodes) => {
                connections.push(ConnectionInfo {
                    tile: tile,
                    pair: [coords[0], coords[1]],
                    nodes: nodes,
                });
            }
            None => (),
        }
    }
    connections
}

fn remove_connected_tiles(map: &TileMap, connections: &Vec<ConnectionInfo>) -> TileMap {
    let mut new_map = map.clone();
    for connection in connections {
        for coord in connection.pair {
            new_map[coord] = None;
        }
    }
    new_map
}

fn collect_connection_single_pair_tiles_loop(map: &TileMap) -> Vec<ConnectionInfo> {
    let mut connections = collect_connection_single_pair_tiles(map);
    let mut current_map = remove_connected_tiles(map, &connections);

    loop {
        let conn = collect_connection_single_pair_tiles(&current_map);
        if conn.len() == 0 {
            break;
        }

        current_map = remove_connected_tiles(&current_map, &conn);
        connections.extend(conn);
    }
    connections
}

fn get_pair_indices(n_items: usize) -> Vec<[usize; 2]> {
    let mut indices: Vec<[usize; 2]> = Vec::new();
    for i in 0..(n_items - 1) {
        for j in (i + 1)..n_items {
            indices.push([i, j]);
        }
    }
    indices
}

fn create_tile_connection_lookup_table(map: &TileMap, maplut: &TileLut) -> ConnectionLut {
    let mut lut: ConnectionLut = HashMap::new();

    for (&tile, coords) in maplut.iter() {
        let mut connections: Vec<ConnectionInfo> = Vec::new();
        for [i, j] in get_pair_indices(coords.len()) {
            match find_connection(map, &coords[i], &coords[j]) {
                Some(nodes) => connections.push(ConnectionInfo {
                    tile: tile,
                    pair: [coords[i], coords[j]],
                    nodes: nodes,
                }),
                _ => (),
            }
        }
        lut.insert(tile, connections);
    }
    lut
}

fn count_unique_coords_in_connections(connections: &Vec<ConnectionInfo>) -> usize {
    let coords: HashSet<Coord> = connections.iter().map(|conn| conn.pair).flatten().collect();
    coords.len()
}

fn get_completed_connections(
    connections: &Vec<ConnectionInfo>,
    n_coords: usize,
) -> Option<Vec<ConnectionInfo>> {
    let mut result: Vec<ConnectionInfo> = Vec::new();
    let mut curr = connections.clone();
    let mut log: Vec<(Vec<ConnectionInfo>, usize)> = Vec::new();
    let mut idx: usize = 0;

    loop {
        if count_unique_coords_in_connections(&curr) + result.len() * 2 != n_coords
            || idx == curr.len()
        {
            match log.pop() {
                Some((conn, i)) if result.len() > 0 => {
                    result.pop();
                    curr = conn;
                    idx = i + 1;
                }
                _ => {
                    return None;
                }
            }
            continue;
        }

        log.push((curr.clone(), idx));
        result.push(curr[idx].clone());

        let target = curr[idx].pair;
        curr = curr
            .iter()
            .cloned()
            .filter(|conn| !(conn.pair.contains(&target[0]) || conn.pair.contains(&target[1])))
            .collect();
        idx = 0;

        if count_unique_coords_in_connections(&result) == n_coords {
            return Some(result);
        }
    }
}

fn collect_completed_connections(map: &TileMap) -> Vec<ConnectionInfo> {
    let maplut = create_tile_lookup_table(map);
    let lut = create_tile_connection_lookup_table(map, &maplut);
    let mut completed: Vec<ConnectionInfo> = Vec::new();

    for (tile, conns) in lut.iter() {
        let n_coords = maplut[tile].len();
        match get_completed_connections(&conns, n_coords) {
            Some(connections) => completed.extend(connections),
            _ => (),
        };
    }

    completed
}

#[derive(Debug)]
pub struct SichuanSolver {
    current: TileMap,
    index_log: Vec<usize>,
    map_log: Vec<TileMap>,
    assumed_connections: Vec<ConnectionInfo>,
    available_connections: Vec<Vec<ConnectionInfo>>,
    confirmed_connections: Vec<Vec<ConnectionInfo>>,
}

#[derive(Debug, PartialEq)]
pub enum SichuanSolverStatus {
    NoAnswerFound,
    InProgress,
    AnswerFound,
}

#[derive(Debug, PartialEq)]
enum SichuanSolverRollbackReason {
    AssumeIndexOutOfRange,
    LogicalInterpolationFailed,
    ValidationFailed,
}

impl SichuanSolver {
    pub fn new(map: &TileMap) -> Self {
        Self {
            current: map.clone(),
            map_log: Vec::new(),
            index_log: Vec::new(),
            assumed_connections: Vec::new(),
            available_connections: Vec::new(),
            confirmed_connections: Vec::new(),
        }
    }

    fn apply_connection_collector(
        &mut self,
        collector: &dyn Fn(&TileMap) -> Vec<ConnectionInfo>,
    ) -> Vec<ConnectionInfo> {
        let conn = collector(&self.current);
        self.current = remove_connected_tiles(&self.current, &conn);
        conn
    }

    fn logically_interpolate_connection(&mut self) -> Option<SichuanSolverRollbackReason> {
        // TODO Return SichuanSolverRollbackReason::LogicalInterpolationFailed if collector fails
        let mut connections: Vec<ConnectionInfo> = Vec::new();

        loop {
            let mut conn: Vec<ConnectionInfo> = Vec::new();
            conn.extend(self.apply_connection_collector(&collect_completed_connections));

            if conn.len() == 0 {
                break;
            }

            connections.extend(conn);
        }

        self.confirmed_connections.push(connections);
        self.map_log.push(self.current.clone());
        None
    }

    fn set_available_connections(&mut self) {
        let maplut = create_tile_lookup_table(&self.current);
        let connlut = create_tile_connection_lookup_table(&self.current, &maplut);
        let mut connnections: Vec<ConnectionInfo> = connlut.into_values().flatten().collect();
        connnections.sort_by(|conn1, conn2| match (conn1.pair, conn2.pair) {
            ([[x1, _], _], [[x2, _], _]) if x1 != x2 => x1.cmp(&x2),
            ([[_, y1], _], [[_, y2], _]) if y1 != y2 => y1.cmp(&y2),
            ([_, [x1, _]], [_, [x2, _]]) if x1 != x2 => x1.cmp(&x2),
            ([_, [_, y1]], [_, [_, y2]]) => y1.cmp(&y2),
        });
        self.available_connections.push(connnections);
    }

    fn should_resume_index(&self) -> bool {
        self.map_log.len() == self.index_log.len()
    }

    fn assume_connection(&mut self) -> Option<SichuanSolverRollbackReason> {
        // Rollbacked
        if self.should_resume_index() {
            *self.index_log.last_mut().unwrap() += 1;
        } else {
            self.index_log.push(0);
        }

        let connection = self
            .available_connections
            .last()
            .unwrap()
            .get(*self.index_log.last().unwrap());

        if let Some(conn) = connection {
            self.assumed_connections.push(conn.clone());
            self.current = remove_connected_tiles(&self.current, &vec![conn.clone()]);
            return None;
        }

        Some(SichuanSolverRollbackReason::AssumeIndexOutOfRange)
    }

    fn is_completed(&self) -> bool {
        Vec::from_iter(self.current.iter().flatten()).len() == 0
    }

    fn validate_current_map(&self) -> Option<SichuanSolverRollbackReason> {
        if self.available_connections.last().unwrap().len() == 0 {
            return Some(SichuanSolverRollbackReason::ValidationFailed);
        }
        None
    }

    fn rollback(&mut self, reason: SichuanSolverRollbackReason) -> Option<SichuanSolverStatus> {
        match reason {
            SichuanSolverRollbackReason::AssumeIndexOutOfRange => match self.map_log.pop() {
                Some(_) => {
                    self.current = self.map_log.last().unwrap().clone();
                    self.index_log.pop();
                    self.assumed_connections.pop();
                    self.available_connections.pop();
                    self.confirmed_connections.pop();
                }
                None => return Some(SichuanSolverStatus::NoAnswerFound),
            },
            SichuanSolverRollbackReason::LogicalInterpolationFailed => {
                self.current = self.map_log.last().unwrap().clone();
            }
            SichuanSolverRollbackReason::ValidationFailed => match self.map_log.pop() {
                Some(_) => {
                    self.current = self.map_log.last().unwrap().clone();
                    self.assumed_connections.pop();
                    self.available_connections.pop();
                    self.confirmed_connections.pop();
                }
                _ => return Some(SichuanSolverStatus::NoAnswerFound),
            },
        }

        None
    }

    fn next(&mut self) -> SichuanSolverStatus {
        if let Some(err) = self.assume_connection() {
            return match self.rollback(err) {
                None => SichuanSolverStatus::InProgress,
                Some(status) => status,
            };
        }

        if let Some(err) = self.logically_interpolate_connection() {
            return match self.rollback(err) {
                None => SichuanSolverStatus::InProgress,
                Some(status) => status,
            };
        }

        if self.is_completed() {
            return SichuanSolverStatus::AnswerFound;
        }

        self.set_available_connections();
        if let Some(err) = self.validate_current_map() {
            return match self.rollback(err) {
                None => SichuanSolverStatus::InProgress,
                Some(status) => status,
            };
        }

        SichuanSolverStatus::InProgress
    }

    fn get_connections(&self) -> Vec<ConnectionInfo> {
        let mut interpolated = self.confirmed_connections.clone();
        let last = interpolated.pop();
        for (int_conns, asm_conn) in interpolated.iter_mut().zip(&self.assumed_connections) {
            int_conns.push(asm_conn.clone());
        }
        interpolated.push(last.unwrap());
        interpolated.into_iter().flatten().collect()
    }

    pub fn solve(&mut self) -> Result<Vec<ConnectionInfo>, SichuanSolverStatus> {
        self.logically_interpolate_connection();
        self.set_available_connections();

        loop {
            match self.next() {
                SichuanSolverStatus::AnswerFound => return Ok(self.get_connections()),
                SichuanSolverStatus::NoAnswerFound => {
                    return Err(SichuanSolverStatus::NoAnswerFound)
                }
                _ => (),
            }
        }
    }
}

#[test]
fn test_create_tile_lookup_table() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(0), Some(1), Some(2)],
        [Some(3), Some(0), Some(0), Some(1)],
        [Some(2), Some(1), Some(1), Some(3)],
    ]);

    let lut = create_tile_lookup_table(&map);
    assert_eq!(lut.get(&0), Some(&vec![[0, 0], [0, 1], [1, 1], [1, 2]]));
    assert_eq!(lut.get(&1), Some(&vec![[0, 2], [1, 3], [2, 1], [2, 2]]));
    assert_eq!(lut.get(&2), Some(&vec![[0, 3], [2, 0]]));
    assert_eq!(lut.get(&3), Some(&vec![[1, 0], [2, 3]]));
    assert_eq!(lut.get(&4), None);
    assert_eq!(lut.len(), 4);
}

#[test]
fn test_collect_connection_single_pair_tiles() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(0), Some(1), Some(2)],
        [Some(3), Some(1), None, Some(2)],
        [None, None, Some(0), Some(4)],
        [Some(0), Some(3), None, None],
        [Some(4), Some(0), Some(0), None],
    ]);
    /*
     * 0 0 1 2
     * 3 1 - 2
     * - - 0 4
     * 0 3 - -
     * 4 0 0 -
     */
    let mut connections = collect_connection_single_pair_tiles(&map);
    connections.sort_by(|a, b| a.tile.cmp(&b.tile));
    assert_eq!(
        connections[0],
        ConnectionInfo {
            tile: 1,
            pair: [[0, 2], [1, 1]],
            nodes: [Some([0, 2]), Some([1, 2]), Some([1, 1]), None],
        }
    );
    assert_eq!(
        connections[1],
        ConnectionInfo {
            tile: 2,
            pair: [[0, 3], [1, 3]],
            nodes: [Some([0, 3]), Some([1, 3]), None, None],
        }
    );
    assert_eq!(
        connections[2],
        ConnectionInfo {
            tile: 3,
            pair: [[1, 0], [3, 1]],
            nodes: [Some([1, 0]), Some([2, 0]), Some([2, 1]), Some([3, 1])],
        }
    );
    assert_eq!(connections.len(), 3);
}

#[test]
fn test_remove_connected_tiles() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(0), Some(1), Some(2)],
        [Some(3), Some(1), Some(4), Some(2)],
        [Some(4), Some(5), Some(5), Some(0)],
        [Some(0), Some(3), Some(0), Some(0)],
    ]);
    /*
     * 0 0 1 2
     * 3 1 4 2
     * 4 5 5 0
     * 0 3 0 0
     */
    let map2: TileMap = arr2(&[
        [Some(0), Some(0), Some(1), None],
        [Some(3), Some(1), Some(4), None],
        [Some(4), None, None, Some(0)],
        [Some(0), Some(3), Some(0), Some(0)],
    ]);
    /*
     * 0 0 1 -
     * 3 1 4 -
     * 4 - - 0
     * 0 3 0 0
     */
    let connections = collect_connection_single_pair_tiles(&map);
    assert_eq!(remove_connected_tiles(&map, &connections), map2);
}

#[test]
fn test_collect_connection_single_pair_tiles_loop() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(0), Some(1), Some(2)],
        [Some(3), Some(1), Some(4), Some(2)],
        [Some(4), Some(5), Some(5), Some(0)],
        [Some(0), Some(3), Some(0), Some(0)],
    ]);
    /*
     * 0 0 1 2
     * 3 1 4 2
     * 4 5 5 0
     * 0 3 0 0
     */
    let mut connections = collect_connection_single_pair_tiles_loop(&map);
    connections.sort_by(|a, b| a.tile.cmp(&b.tile));
    assert_eq!(connections.len(), 5);
    assert_eq!(
        connections[0],
        ConnectionInfo {
            tile: 1,
            pair: [[0, 2], [1, 1]],
            nodes: [Some([0, 2]), Some([1, 2]), Some([1, 1]), None],
        }
    );
    assert_eq!(
        connections[1],
        ConnectionInfo {
            tile: 2,
            pair: [[0, 3], [1, 3]],
            nodes: [Some([0, 3]), Some([1, 3]), None, None],
        }
    );
    assert_eq!(
        connections[2],
        ConnectionInfo {
            tile: 3,
            pair: [[1, 0], [3, 1]],
            nodes: [Some([1, 0]), Some([2, 0]), Some([2, 1]), Some([3, 1])],
        }
    );
    assert_eq!(
        connections[3],
        ConnectionInfo {
            tile: 4,
            pair: [[1, 2], [2, 0]],
            nodes: [Some([1, 2]), Some([2, 2]), Some([2, 0]), None],
        }
    );
    assert_eq!(
        connections[4],
        ConnectionInfo {
            tile: 5,
            pair: [[2, 1], [2, 2]],
            nodes: [Some([2, 1]), Some([2, 2]), None, None],
        }
    );
}

#[test]
fn test_get_pair_indices() {
    assert_eq!(get_pair_indices(2), [[0, 1]]);
    assert_eq!(
        get_pair_indices(4),
        [[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]]
    );
    assert_eq!(
        get_pair_indices(6),
        [
            [0, 1],
            [0, 2],
            [0, 3],
            [0, 4],
            [0, 5],
            [1, 2],
            [1, 3],
            [1, 4],
            [1, 5],
            [2, 3],
            [2, 4],
            [2, 5],
            [3, 4],
            [3, 5],
            [4, 5]
        ]
    );
}

#[test]
fn test_create_tile_connection_lookup_table() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(0), Some(1), Some(2)],
        [Some(3), Some(1), Some(4), Some(2)],
        [Some(4), Some(5), Some(5), Some(0)],
        [Some(0), Some(3), Some(0), Some(0)],
    ]);
    /*
     * 0 0 1 2
     * 3 1 4 2
     * 4 5 5 0
     * 0 3 0 0
     */
    let maplut = create_tile_lookup_table(&map);
    let lut = create_tile_connection_lookup_table(&map, &maplut);
    assert_eq!(lut.len(), 6);
    assert_eq!(
        lut.get(&0),
        Some(&vec![
            ConnectionInfo {
                tile: 0,
                pair: [[0, 0], [0, 1]],
                nodes: [Some([0, 0]), Some([0, 1]), None, None],
            },
            ConnectionInfo {
                tile: 0,
                pair: [[2, 3], [3, 3]],
                nodes: [Some([2, 3]), Some([3, 3]), None, None],
            },
            ConnectionInfo {
                tile: 0,
                pair: [[3, 2], [3, 3]],
                nodes: [Some([3, 2]), Some([3, 3]), None, None],
            },
        ])
    );
    assert_eq!(lut.get(&1), Some(&vec![]));
    assert_eq!(
        lut.get(&2),
        Some(&vec![ConnectionInfo {
            tile: 2,
            pair: [[0, 3], [1, 3]],
            nodes: [Some([0, 3]), Some([1, 3]), None, None],
        }])
    );
    assert_eq!(lut.get(&3), Some(&vec![]));
    assert_eq!(lut.get(&4), Some(&vec![]));
    assert_eq!(
        lut.get(&5),
        Some(&vec![ConnectionInfo {
            tile: 5,
            pair: [[2, 1], [2, 2]],
            nodes: [Some([2, 1]), Some([2, 2]), None, None],
        }])
    );
}

#[test]
fn test_count_unique_coords_in_connections() {
    let build_connections = |pairs: &[[Coord; 2]]| -> Vec<ConnectionInfo> {
        pairs
            .iter()
            .map(|pair| ConnectionInfo {
                tile: 0,
                pair: pair.clone(),
                nodes: [None, None, None, None],
            })
            .collect()
    };

    let coords1 = [[[0, 0], [0, 1]], [[0, 2], [0, 3]]];
    let coords2 = [[[0, 0], [0, 1]], [[0, 0], [0, 2]]];
    let coords3 = [
        [[0, 0], [0, 1]],
        [[0, 1], [0, 2]],
        [[0, 2], [0, 3]],
        [[0, 1], [0, 3]],
    ];

    assert_eq!(
        count_unique_coords_in_connections(&build_connections(&coords1)),
        4
    );
    assert_eq!(
        count_unique_coords_in_connections(&build_connections(&coords2)),
        3
    );
    assert_eq!(
        count_unique_coords_in_connections(&build_connections(&coords3)),
        4
    );
}

#[test]
fn test_get_completed_connections() {
    let build_connections = |pairs: &[[Coord; 2]]| -> Vec<ConnectionInfo> {
        pairs
            .iter()
            .map(|pair| ConnectionInfo {
                tile: 0,
                pair: pair.clone(),
                nodes: [None, None, None, None],
            })
            .collect()
    };

    let conn1 = build_connections(&[[[0, 0], [0, 1]], [[0, 2], [0, 3]]]);
    assert_eq!(get_completed_connections(&conn1, 4), Some(conn1.clone()));
    assert_eq!(get_completed_connections(&conn1, 6), None);

    let conn2 = build_connections(&[[[0, 0], [0, 1]], [[0, 0], [0, 2]], [[0, 0], [0, 3]]]);
    assert_eq!(get_completed_connections(&conn2, 4), None);

    let conn3 = build_connections(&[[[0, 0], [0, 1]], [[0, 0], [0, 2]], [[0, 1], [0, 3]]]);
    let expected3 = build_connections(&[[[0, 0], [0, 2]], [[0, 1], [0, 3]]]);
    assert_eq!(get_completed_connections(&conn3, 4), Some(expected3));
}

#[test]
fn test_collect_completed_connections() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(0), Some(0), Some(0)],
        [Some(1), Some(1), Some(2), Some(3)],
        [Some(2), Some(1), Some(1), Some(4)],
        [Some(4), Some(3), Some(4), Some(4)],
    ]);
    /*
     * 0 0 0 0
     * 1 1 2 3
     * 2 1 1 4
     * 4 3 4 4
     */

    let expected: HashSet<&ConnectionInfo> = [
        ConnectionInfo {
            pair: [[0, 0], [0, 1]],
            tile: 0,
            nodes: [Some([0, 0]), Some([0, 1]), None, None],
        },
        ConnectionInfo {
            pair: [[0, 2], [0, 3]],
            tile: 0,
            nodes: [Some([0, 2]), Some([0, 3]), None, None],
        },
        ConnectionInfo {
            pair: [[1, 0], [1, 1]],
            tile: 1,
            nodes: [Some([1, 0]), Some([1, 1]), None, None],
        },
        ConnectionInfo {
            pair: [[2, 1], [2, 2]],
            tile: 1,
            nodes: [Some([2, 1]), Some([2, 2]), None, None],
        },
    ]
    .iter()
    .collect();
    let result = collect_completed_connections(&map);
    let actual: HashSet<&ConnectionInfo> = result.iter().collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_sichuan_solver_logically_interpolate_connections() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(0), Some(0), Some(0)],
        [Some(1), Some(1), Some(2), Some(3)],
        [Some(2), Some(1), Some(1), Some(4)],
        [Some(4), Some(3), Some(4), Some(4)],
    ]);
    /*
     * 0 0 0 0
     * 1 1 2 3
     * 2 1 1 4
     * 4 3 4 4
     */

    let mut solver = SichuanSolver::new(&map);
    solver.logically_interpolate_connection();

    let expected1: HashSet<&ConnectionInfo> = [
        ConnectionInfo {
            pair: [[0, 0], [0, 1]],
            tile: 0,
            nodes: [Some([0, 0]), Some([0, 1]), None, None],
        },
        ConnectionInfo {
            pair: [[0, 2], [0, 3]],
            tile: 0,
            nodes: [Some([0, 2]), Some([0, 3]), None, None],
        },
        ConnectionInfo {
            pair: [[1, 0], [1, 1]],
            tile: 1,
            nodes: [Some([1, 0]), Some([1, 1]), None, None],
        },
        ConnectionInfo {
            pair: [[2, 1], [2, 2]],
            tile: 1,
            nodes: [Some([2, 1]), Some([2, 2]), None, None],
        },
    ]
    .iter()
    .collect();
    let expected2: HashSet<&ConnectionInfo> = [
        ConnectionInfo {
            pair: [[1, 2], [2, 0]],
            tile: 2,
            nodes: [Some([1, 2]), Some([2, 2]), Some([2, 0]), None],
        },
        ConnectionInfo {
            pair: [[1, 3], [3, 1]],
            tile: 3,
            nodes: [Some([3, 1]), Some([0, 1]), Some([0, 3]), Some([1, 3])],
        },
    ]
    .iter()
    .collect();
    let expected3: HashSet<&ConnectionInfo> = [
        ConnectionInfo {
            pair: [[2, 3], [3, 0]],
            tile: 4,
            nodes: [Some([2, 3]), Some([2, 0]), Some([3, 0]), None],
        },
        ConnectionInfo {
            pair: [[3, 2], [3, 3]],
            tile: 4,
            nodes: [Some([3, 2]), Some([3, 3]), None, None],
        },
    ]
    .iter()
    .collect();

    let mut connections = solver.confirmed_connections[0].clone();
    let conn3 = connections.split_off(6);
    let actual3: HashSet<&ConnectionInfo> = conn3.iter().collect();
    let conn2 = connections.split_off(4);
    let actual2: HashSet<&ConnectionInfo> = conn2.iter().collect();
    let actual1: HashSet<&ConnectionInfo> = connections.iter().collect();

    assert_eq!(actual1, expected1);
    assert_eq!(actual2, expected2);
    assert_eq!(actual3, expected3);
}

#[test]
fn test_sichuan_solver_is_completed() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(0), Some(0), Some(0)],
        [Some(1), Some(1), Some(2), Some(3)],
        [Some(2), Some(1), Some(1), Some(4)],
        [Some(4), Some(3), Some(4), Some(4)],
    ]);
    /*
     * 0 0 0 0
     * 1 1 2 3
     * 2 1 1 4
     * 4 3 4 4
     */

    let mut solver = SichuanSolver::new(&map);
    assert_eq!(solver.is_completed(), false);

    solver.logically_interpolate_connection();
    assert_eq!(solver.is_completed(), true);

    let map2: TileMap = arr2(&[[Some(0), Some(1)], [Some(1), Some(0)]]);
    /*
     * 0 1
     * 1 0
     */
    let mut solver2 = SichuanSolver::new(&map2);
    assert_eq!(solver2.is_completed(), false);

    solver2.logically_interpolate_connection();
    assert_eq!(solver2.is_completed(), false);
}

#[test]
fn test_sichuan_solver_set_available_connections() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(0), Some(1), Some(0)],
        [Some(0), Some(1), Some(1), Some(2)],
        [Some(1), Some(2), Some(2), Some(3)],
        [Some(3), Some(2), Some(3), Some(3)],
    ]);
    /*
     * 0 0 1 0
     * 0 1 1 2
     * 1 2 2 3
     * 3 2 3 3
     */
    let mut solver = SichuanSolver::new(&map);
    solver.logically_interpolate_connection();
    solver.set_available_connections();

    let expected: HashSet<&ConnectionInfo> = [
        ConnectionInfo {
            pair: [[0, 0], [0, 1]],
            tile: 0,
            nodes: [Some([0, 0]), Some([0, 1]), None, None],
        },
        ConnectionInfo {
            pair: [[0, 0], [1, 0]],
            tile: 0,
            nodes: [Some([0, 0]), Some([1, 0]), None, None],
        },
        ConnectionInfo {
            pair: [[0, 2], [1, 2]],
            tile: 1,
            nodes: [Some([0, 2]), Some([1, 2]), None, None],
        },
        ConnectionInfo {
            pair: [[1, 1], [1, 2]],
            tile: 1,
            nodes: [Some([1, 1]), Some([1, 2]), None, None],
        },
        ConnectionInfo {
            pair: [[2, 1], [2, 2]],
            tile: 2,
            nodes: [Some([2, 1]), Some([2, 2]), None, None],
        },
        ConnectionInfo {
            pair: [[2, 1], [3, 1]],
            tile: 2,
            nodes: [Some([2, 1]), Some([3, 1]), None, None],
        },
        ConnectionInfo {
            pair: [[2, 3], [3, 3]],
            tile: 3,
            nodes: [Some([2, 3]), Some([3, 3]), None, None],
        },
        ConnectionInfo {
            pair: [[3, 2], [3, 3]],
            tile: 3,
            nodes: [Some([3, 2]), Some([3, 3]), None, None],
        },
    ]
    .iter()
    .collect();

    let connections = solver.available_connections.last().clone().unwrap();
    let actual: HashSet<&ConnectionInfo> = connections.iter().collect();
    assert_eq!(actual, expected);

    assert_eq!(solver.current, map);
    assert_eq!(solver.map_log.len(), 1);
    assert_eq!(solver.confirmed_connections, vec![vec![]]);
    assert_eq!(solver.assumed_connections, vec![]);
    assert_eq!(solver.index_log, vec![]);
}

fn test_sichuan_solver_assume_connection() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(0), Some(1), Some(0)],
        [Some(0), Some(1), Some(1), Some(2)],
        [Some(1), Some(2), Some(2), Some(3)],
        [Some(3), Some(2), Some(3), Some(3)],
    ]);
    /*
     * 0 0 1 0
     * 0 1 1 2
     * 1 2 2 3
     * 3 2 3 3
     */
    let mut solver = SichuanSolver::new(&map);
    solver.logically_interpolate_connection();
    solver.set_available_connections();

    assert_eq!(solver.assumed_connections, vec![]);
    solver.assume_connection();
    assert_eq!(
        solver.assumed_connections,
        vec![ConnectionInfo {
            pair: [[0, 0], [0, 1]],
            tile: 0,
            nodes: [Some([0, 0]), Some([0, 1]), None, None],
        },]
    );
    assert_eq!(solver.index_log, vec![0]);
    assert_ne!(solver.current, map);
    assert_eq!(solver.map_log.len(), 1);
    assert_eq!(solver.confirmed_connections, vec![vec![]]);

    let newmap: TileMap = arr2(&[
        [None, None, Some(1), Some(0)],
        [Some(0), Some(1), Some(1), Some(2)],
        [Some(1), Some(2), Some(2), Some(3)],
        [Some(3), Some(2), Some(3), Some(3)],
    ]);
    assert_eq!(solver.current, newmap);

    let map2: TileMap = arr2(&[[Some(0), Some(1)], [Some(1), Some(0)]]);
    /*
     * 0 1
     * 1 0
     */
    let mut solver2 = SichuanSolver::new(&map2);
    solver2.logically_interpolate_connection();
    solver2.set_available_connections();
    assert_eq!(
        solver2.assume_connection(),
        Some(SichuanSolverRollbackReason::AssumeIndexOutOfRange)
    );
}

#[test]
fn test_sichuan_solver_validate_current_map() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(0), Some(1), Some(0)],
        [Some(0), Some(1), Some(1), Some(2)],
        [Some(3), Some(2), Some(2), Some(3)],
        [Some(3), Some(2), Some(3), Some(1)],
    ]);
    /*
     * 0 0 1 0
     * 0 1 1 2
     * 3 2 2 3
     * 3 2 3 1
     */
    let mut solver = SichuanSolver::new(&map);
    solver.logically_interpolate_connection();
    solver.set_available_connections();
    solver.assume_connection();
    solver.logically_interpolate_connection();
    solver.set_available_connections();
    assert_eq!(solver.is_completed(), false);
    assert_eq!(solver.validate_current_map(), None);

    let map2: TileMap = arr2(&[
        [None, None, None, None],
        [Some(0), Some(0), Some(0), Some(1)],
        [None, None, Some(1), Some(0)],
    ]);
    /*
     * x x x x
     * 0 0 0 1
     * x x 1 0
     */
    let mut solver2 = SichuanSolver::new(&map2);
    solver2.logically_interpolate_connection();
    solver2.set_available_connections();
    solver2.assume_connection();
    solver2.logically_interpolate_connection();
    solver2.set_available_connections();
    assert_eq!(solver2.is_completed(), false);
    assert_eq!(
        solver2.validate_current_map(),
        Some(SichuanSolverRollbackReason::ValidationFailed)
    );
}

#[test]
fn test_sichuan_solver_rollback() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(0), Some(0), Some(1)],
        [None, None, Some(1), Some(0)],
    ]);
    /*
     * 0 0 0 1
     * x x 1 0
     */
    let mut solver = SichuanSolver::new(&map);
    solver.logically_interpolate_connection();
    solver.set_available_connections();
    solver.assume_connection();
    solver.logically_interpolate_connection();
    solver.set_available_connections();
    assert_eq!(solver.is_completed(), false);

    let reason = solver.validate_current_map();
    solver.rollback(reason.unwrap());
    assert_eq!(solver.map_log.len(), 1);
    assert_eq!(solver.index_log.len(), 1);
    assert_eq!(solver.confirmed_connections.len(), 1);
    assert_eq!(solver.available_connections.len(), 1);
    assert_eq!(solver.assumed_connections.len(), 0);
    assert_eq!(solver.should_resume_index(), true);
    assert_eq!(&solver.current, solver.map_log.last().unwrap());

    solver.assume_connection();
    assert_eq!(solver.assumed_connections.len(), 1);
    assert_eq!(solver.index_log, vec![1]);
    solver.logically_interpolate_connection();
    solver.set_available_connections();
    assert!(solver.is_completed());

    let map2: TileMap = arr2(&[
        [Some(0), Some(1), None, Some(0)],
        [Some(1), Some(1), Some(2), Some(4)],
        [Some(2), Some(3), Some(0), Some(3)],
        [None, Some(0), Some(1), Some(4)],
    ]);
    /*
     * 0 1 x 0
     * 1 1 2 4
     * 2 3 0 3
     * x 0 1 4
     */
    let mut solver2 = SichuanSolver::new(&map2);
    solver2.logically_interpolate_connection();
    solver2.set_available_connections();
    solver2.assume_connection();
    solver2.logically_interpolate_connection();
    solver2.set_available_connections();
    assert_eq!(solver2.is_completed(), false);
    assert_eq!(solver2.validate_current_map(), None);

    assert_eq!(solver2.should_resume_index(), false);
    solver2.assume_connection();
    solver2.logically_interpolate_connection();
    solver2.set_available_connections();

    let reason2 = solver2.validate_current_map();
    solver2.rollback(reason2.unwrap());
    assert_eq!(solver2.map_log.len(), 2);
    assert_eq!(solver2.index_log.len(), 2);
    assert_eq!(solver2.confirmed_connections.len(), 2);
    assert_eq!(solver2.available_connections.len(), 2);
    assert_eq!(solver2.assumed_connections.len(), 1);
    assert_eq!(solver2.should_resume_index(), true);
    assert_eq!(&solver2.current, solver2.map_log.last().unwrap());

    let reason3 = solver2.assume_connection();
    solver2.rollback(reason3.unwrap());
    assert_eq!(solver2.map_log.len(), 1);
    assert_eq!(solver2.index_log.len(), 1);
    assert_eq!(solver2.confirmed_connections.len(), 1);
    assert_eq!(solver2.available_connections.len(), 1);
    assert_eq!(solver2.assumed_connections.len(), 0);
    assert_eq!(solver2.should_resume_index(), true);
    assert_eq!(&solver2.current, solver2.map_log.last().unwrap());

    solver2.assume_connection();
    solver2.logically_interpolate_connection();
    solver2.set_available_connections();
    assert_eq!(solver2.is_completed(), true);
}

#[test]
fn test_sichuan_solver_next() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(1), None, Some(0)],
        [Some(1), Some(1), Some(2), Some(4)],
        [Some(2), Some(3), Some(0), Some(3)],
        [None, Some(0), Some(1), Some(4)],
    ]);
    /*
     * 0 1 x 0
     * 1 1 2 4
     * 2 3 0 3
     * x 0 1 4
     */

    let mut solver = SichuanSolver::new(&map);
    solver.logically_interpolate_connection();
    solver.set_available_connections();
    let result1 = solver.next();
    assert_eq!(result1, SichuanSolverStatus::InProgress);
    assert_eq!(solver.map_log.len(), 2);
    assert_eq!(solver.index_log.len(), 1);
    assert_eq!(solver.confirmed_connections.len(), 2);
    assert_eq!(solver.available_connections.len(), 2);
    assert_eq!(solver.assumed_connections.len(), 1);
    assert_eq!(solver.should_resume_index(), false);

    let result2 = solver.next();
    assert_eq!(result2, SichuanSolverStatus::InProgress);
    assert_eq!(solver.map_log.len(), 2);
    assert_eq!(solver.index_log.len(), 2);
    assert_eq!(solver.confirmed_connections.len(), 2);
    assert_eq!(solver.available_connections.len(), 2);
    assert_eq!(solver.assumed_connections.len(), 1);
    assert_eq!(solver.should_resume_index(), true);
    assert_eq!(&solver.current, solver.map_log.last().unwrap());

    let result3 = solver.next();
    assert_eq!(result3, SichuanSolverStatus::InProgress);
    assert_eq!(solver.map_log.len(), 1);
    assert_eq!(solver.index_log.len(), 1);
    assert_eq!(solver.confirmed_connections.len(), 1);
    assert_eq!(solver.available_connections.len(), 1);
    assert_eq!(solver.assumed_connections.len(), 0);
    assert_eq!(solver.should_resume_index(), true);
    assert_eq!(&solver.current, solver.map_log.last().unwrap());

    let result4 = solver.next();
    assert_eq!(result4, SichuanSolverStatus::AnswerFound);
}

#[test]
fn test_sichuan_solver_solve() {
    use ndarray::{arr2, Array2};
    let map: TileMap = arr2(&[
        [Some(0), Some(1), None, Some(0)],
        [Some(1), Some(1), Some(2), Some(4)],
        [Some(2), Some(3), Some(0), Some(3)],
        [None, Some(0), Some(1), Some(4)],
    ]);
    /*
     * 0 1 x 0
     * 1 1 2 4
     * 2 3 0 3
     * x 0 1 4
     */

    let mut solver = SichuanSolver::new(&map);
    let result = solver.solve();
    assert_eq!(result.unwrap().len(), 7);
    assert_eq!(solver.current, Array2::from_elem((4, 4), None));
}
