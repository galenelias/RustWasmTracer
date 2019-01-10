// import * as wasm from "RustWasmTracer";

import { Universe, Cell } from "RustWasmTracer";

const CELL_SIZE = 3; // px
const BORDER_SIZE = 0;
const GRID_COLOR = "#CCCCCC";

const SAND_COLOR = "#FFFFFF";
const CLAY_COLOR = "#808080";
const SETTLED_WATER_COLOR = "#0000FF";
const FLOWING_WATER_COLOR = "#3399FF";

// Construct the universe, and get its width and height.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + BORDER_SIZE) * height + 1;
canvas.width = (CELL_SIZE + BORDER_SIZE) * width + 1;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
  universe.tick();

  drawGrid();
  drawCells();

  requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
  if (BORDER_SIZE == 0) {
    return;
  }

	ctx.beginPath();
	ctx.strokeStyle = GRID_COLOR;
  
	// Vertical lines.
	for (let i = 0; i <= width; i++) {
	  ctx.moveTo(i * (CELL_SIZE + BORDER_SIZE) + 1, 0);
	  ctx.lineTo(i * (CELL_SIZE + BORDER_SIZE) + 1, (CELL_SIZE + BORDER_SIZE) * height + 1);
	}
  
	// Horizontal lines.
	for (let j = 0; j <= height; j++) {
	  ctx.moveTo(0,                                     j * (CELL_SIZE + BORDER_SIZE) + 1);
	  ctx.lineTo((CELL_SIZE + BORDER_SIZE) * width + 1, j * (CELL_SIZE + BORDER_SIZE) + 1);
	}
  
	ctx.stroke();
};

// Import the WebAssembly memory at the top of the file.
import { memory } from "RustWasmTracer/RustWasmTracer_bg";

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      switch (cells[idx]) {
        case Cell.Sand: ctx.fillStyle = SAND_COLOR; break;
        case Cell.Clay: ctx.fillStyle = CLAY_COLOR; break;
        case Cell.FlowingWater: ctx.fillStyle = FLOWING_WATER_COLOR; break;
        case Cell.SettledWater: ctx.fillStyle = SETTLED_WATER_COLOR; break;
      }

      ctx.fillRect(
        col * (CELL_SIZE + BORDER_SIZE) + BORDER_SIZE,
        row * (CELL_SIZE + BORDER_SIZE) + BORDER_SIZE,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);
