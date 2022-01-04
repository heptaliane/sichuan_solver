use ndarray::{Array2, s};
use super::components::TileMap;


fn pad_tilemap(map: &TileMap) -> TileMap {
    let shape = map.shape();
    let mut padmap: TileMap = Array2::from_elem((shape[0] + 2, shape[1] + 2), None);
    padmap.slice_mut(s![1..shape[0] + 1, 1..shape[1] + 1]).assign(&map);
    padmap
}

#[test]
fn test_pad_tilemap() {
    use ndarray::arr2;
    let map: TileMap = arr2(&[
        [Some(0), Some(1), Some(2)],
        [Some(2), Some(0), Some(1)],
    ]);
    let padmap: TileMap = arr2(&[
        [None, None, None, None, None],
        [None, Some(0), Some(1), Some(2), None],
        [None, Some(2), Some(0), Some(1), None],
        [None, None, None, None, None],
    ]);
    assert_eq!(padmap, pad_tilemap(&map));
}
