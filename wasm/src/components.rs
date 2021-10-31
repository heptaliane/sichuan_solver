use ndarray::Array2;

pub type Tile = u8;
pub type CoordElement = usize;
pub type Coord = [CoordElement; 2];
pub type TypedCoordElement = i32;
pub type TypedCoord = [TypedCoordElement; 2];
pub type Grid = [Coord; 2];
pub type Nodes = [Option<Coord>; 4];
pub type TileMap = Array2<Option<Tile>>;
