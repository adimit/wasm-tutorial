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
const toggle = document.getElementById("toggle-button");
const step = document.getElementById("step-button");

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

const cellsPtr = universe.cells();
const bitsize = 16;
const cells = new Uint16Array(memory.buffer, cellsPtr, Math.ceil(width * height / bitsize));

const drawCells = () => {
  const getCell = (index) => {
    const cellptr = Math.floor(index / bitsize);
    const bitptr = index % bitsize;
    const mask = 0x8000 >>> bitptr;
    return (cells[cellptr] & mask) > 0
  };

  for (let i = 0; i < width; i++) {
    for (let j = 0; j < height; j++) {
      const cell = getCell(i + j * height); // universe.get(i,j);
      ctx.fillStyle = cell ? ALIVE_COLOR : DEAD_COLOR;
      ctx.fillRect(i * (CELL_SIZE + 1) + 1, j * (CELL_SIZE + 1) + 1, CELL_SIZE, CELL_SIZE);
    }
  }
}

drawGrid();
drawCells();
let iterations = 0;
counter.innerText = iterations;

const render = async () => {
  drawCells();
  counter.innerText = iterations++;
  // console.log(cells.reduce((acc, cell) => [...acc, cell.toString(2).padStart(bitsize, "0")], []).join(", "));
  universe.tick();
};

let nextFrame = requestAnimationFrame(render);
const startTicker = () => setInterval(
  () => { nextFrame = requestAnimationFrame(render);},
  500
);

let ticks = startTicker();
const pause = () => {
    toggle.innerText = "▶";
    clearInterval(ticks);
    ticks = null;
    cancelAnimationFrame(nextFrame);
    nextFrame = null;
};

const play = () => {
    toggle.innerText = "⏸";
    ticks = startTicker();
}

toggle.onclick = () => {
  if (ticks) {
    pause();
  } else {
    play();
  }
};

step.onclick = () => {
  if (ticks) {
    pause();
  } else {
    requestAnimationFrame(render);
  }
}
