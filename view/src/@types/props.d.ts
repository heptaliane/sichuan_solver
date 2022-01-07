declare module 'sichuan-solver-props' {
  import {TileElement} from 'sichuan-solver-tiles';

  export interface EditableTilemapProps {
    rows: number,
    cols: number,
    expandRatio: number,
    tiles: TileElement[],
    onClick: (args: any) => void,
  }
}
