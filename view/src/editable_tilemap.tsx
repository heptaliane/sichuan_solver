import React from 'react';
import {drawTileImage, tileWidth, tileHeight} from './tile_images';

import {TileElement} from 'sichuan-solver-tiles';
import {EditableTilemapProps} from 'sichuan-solver-props';
import {TileClickEvent} from 'sichuan-solver-event';

const defaultSize: number = 6;
const defaultExpandRatio: number = 2;


const EditableTilemap = function(props: EditableTilemapProps) {
  const canvasElem: React.MutableRefObject<HTMLCanvasElement | null> =
    React.useRef(null);

  React.useEffect(() => {
    if (canvasElem.current === null) {
      return;
    }

    const ctx: CanvasRenderingContext2D | null =
      canvasElem.current.getContext('2d');
    if (ctx === null) {
      return;
    }

    props.tiles.forEach((tile: TileElement) => {
      drawTileImage(ctx, tile);
    });
  });

  const handleClick: React.MouseEventHandler<HTMLCanvasElement> =
    function(e: React.MouseEvent) {
      const x: number = Math.floor(e.clientX / tileWidth / props.expandRatio);
      const y: number = Math.floor(e.clientY / tileHeight / props.expandRatio);
      props.onClick({
        x: x, y: y,
      });
    };

  return (
    <div>
      <canvas
        ref={canvasElem}
        width={tileWidth * props.cols}
        height={tileHeight * props.rows}
        style={{
          width: tileWidth * props.cols * props.expandRatio,
          height: tileHeight * props.rows * props.expandRatio,
        }}
        onClick={handleClick}
      />
    </div>
  );
};


EditableTilemap.defaultProps = {
  rows: defaultSize,
  cols: defaultSize,
  expandRatio: defaultExpandRatio,
  tiles: [],
  onClick: (args: TileClickEvent) => {
    // Nothing to do
  },
};

export default EditableTilemap;
