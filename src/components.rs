use std::collections::HashMap;

pub type Tile = u8;
pub type CoordElement = usize;
pub type Coord = [CoordElement; 2];
pub type CoordDeltaElement = i32;
pub type CoordDelta = [CoordDeltaElement; 2];
pub type Grid = [Coord; 2];
pub type Nodes = Vec<Coord>;
pub type TileMap = HashMap<Coord, Tile>;
