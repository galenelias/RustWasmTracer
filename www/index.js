// import * as wasm from "RustWasmTracer";

import { Scene, Cell } from "RustWasmTracer";

// Construct the scene, and get its width and height.
const scene = Scene.new();
const width = scene.width();
const height = scene.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("canvas");
canvas.height = height;
canvas.width = width;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
  scene.tick();

  drawCells();
  requestAnimationFrame(renderLoop);
};

// Import the WebAssembly memory at the top of the file.
import { memory } from "RustWasmTracer/RustWasmTracer_bg";

const getIndex = (row, column) => {
  return row * width + column;
};


const drawCells = () => {
  const cellsPtr = scene.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height * 4);

  var imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
  var data = imageData.data;
  console.log("Rendering: height = " + height + ", width = " + width + ", len = ", data.length + ", cells = ");// + cells);

  for (var i = 0; i < data.length; i += 1) {
    data[i] = cells[i];
  }
    


  ctx.putImageData(imageData, 0, 0);
};

scene.tick();
drawCells();

requestAnimationFrame(renderLoop);
