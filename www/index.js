import * as wasm from "wasm-gol";
import { memory } from "wasm-gol/wasm_gol_bg";

const CELL_SIZE = 20; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = wasm.create_universe();
const width = universe.width();
const height = universe.height();
const canvas = document.getElementById("game-of-life-canvas");
const counter = document.getElementById("counter");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');


function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
}

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  for (let i = 0; i < width; i++) {
    for (let j = 0; j < height; j++) {
      const cell = universe.get(i,j);
      ctx.fillStyle = cell ? ALIVE_COLOR : DEAD_COLOR;
      ctx.fillRect(i * (CELL_SIZE + 1) + 1, j * (CELL_SIZE + 1) + 1, CELL_SIZE, CELL_SIZE);
    }
  }
}

drawGrid();
drawCells();
let iterations = 0;
counter.innerText = iterations;

const renderLoop = async () => {
  await sleep(100);
  drawCells();
  universe.tick();
  counter.innerText = iterations++;

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
