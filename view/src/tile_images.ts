import tileData from './data/tiles.json';
import {TileConfiguration, TileElement} from 'sichuan-solver-tiles';


const tileWidth: number = 40;
const tileHeight: number = 50;
const tileFontSize: number = 28;
const tileOffset: number = 1;
const lineWidth: number = tileOffset * 2;
const lineColor: string = 'black';
const bgColor: string = 'white';
const defaultTileConfiguration: TileConfiguration = {
  text: '',
  color: 'black',
};

const tileImages: readonly HTMLCanvasElement[] =
  Object.freeze(tileData.map((conf) => {
    return Object.assign({}, defaultTileConfiguration, conf);
  }).map((conf: TileConfiguration) => {
    const canvas: HTMLCanvasElement = document.createElement('canvas');
    canvas.width = tileWidth + tileOffset * 2;
    canvas.height = tileHeight + tileOffset * 2;

    const ctx: CanvasRenderingContext2D | null = canvas.getContext('2d');
    if (ctx !== null) {
      ctx.fillStyle = bgColor;
      ctx.strokeStyle = lineColor;
      ctx.lineWidth = lineWidth;
      ctx.fillRect(tileOffset, tileOffset, tileWidth, tileHeight);
      ctx.strokeRect(tileOffset, tileOffset, tileWidth, tileHeight);

      ctx.fillStyle = conf.color;
      ctx.font = `${tileFontSize}px serif`;
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillText(conf.text, canvas.width * 0.5, canvas.height * 0.5);
    }

    return canvas;
  }));

const drawTileImage =
  function(ctx: CanvasRenderingContext2D, tile: TileElement) {
    const image: HTMLCanvasElement = tileImages[tile.value];
    const x: number = tile.x * tileWidth - tileOffset;
    const y: number = tile.y * tileHeight - tileOffset;
    ctx.drawImage(image, x, y);
  };

export {
  drawTileImage,
  tileWidth,
  tileHeight,
};
