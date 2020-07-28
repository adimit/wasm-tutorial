import * as wasm from "wasm-gol";
import { memory } from "wasm-gol/wasm_gol_bg";

const bitsize = 16;
const bitmask = 0x8000;
const CELL_SIZE = 20; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const instantiateFromUniverse = (universe) => {
  const width = universe.width();
  const height = universe.height();
  const cellsPtr = universe.cells();
  return {
    universe,
    width,
    // we proxy tick and flip, as we're hiding the rust object
    tick: () => universe.tick(),
    flip: (x,y) => universe.flip(x,y),
    height,
    iterations: 0,
    cells: new Uint16Array(memory.buffer, cellsPtr, Math.ceil(width * height / bitsize)),
  };
}

let universe = instantiateFromUniverse(wasm.create_random_universe());
const canvas = document.getElementById("game-of-life-canvas");
const counter = document.getElementById("counter");
const clear = document.getElementById("clear-button");
const randomise = document.getElementById("random-start-button");
const updateInterval = document.getElementById("update-interval");
canvas.height = (CELL_SIZE + 1) * universe.height + 1;
canvas.width = (CELL_SIZE + 1) * universe.width + 1;

const ctx = canvas.getContext('2d');
const toggle = document.getElementById("toggle-button");
const step = document.getElementById("step-button");

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= universe.width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * universe.height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= universe.height; j++) {
    ctx.moveTo(0,                                    j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * universe.width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
}

const drawCell = (x,y) => ctx.fillRect(x * (CELL_SIZE + 1) + 1, y * (CELL_SIZE + 1) + 1, CELL_SIZE, CELL_SIZE);
const drawCells = () => {
  const getCell = (index) => {
    const cellptr = Math.floor(index / bitsize);
    const bitptr = index % bitsize;
    const n = bitmask >>> bitptr;
    return (universe.cells[cellptr] & n) === n;
  };
  counter.innerText = universe.iterations;

  for (let x = 0; x < universe.width; x++) {
    for (let y = 0; y < universe.height; y++) {
      const cell = getCell(x + y * universe.height);
      ctx.fillStyle = cell ? ALIVE_COLOR : DEAD_COLOR;
      drawCell(x,y);
    }
  }
}

const getMouseCoordinates = (event) => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const y = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), universe.height - 1);
  const x = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), universe.width - 1);
  return [x,y];
}

canvas.onpointermove = event => {
  const [x,y] = getMouseCoordinates(event);
  drawCells();
  ctx.fillStyle = "#2222FFAA";
  drawCell(x,y);
}

canvas.onclick = event => {
  const [x,y] = getMouseCoordinates(event);

  universe.flip(x, y);

  drawCells();
};

drawGrid();
drawCells();

const getAnimationInterval = () => Math.pow(2, updateInterval.valueAsNumber);

const render = () => {
    universe.tick();
    drawCells();
    counter.innerText = universe.iterations++;
    // console.log(universe.cells.reduce((acc, cell) => [...acc, cell.toString(2).padStart(bitsize, "0")], []).join(", "));
};

let state;
let timeout = null;

const renderLoop = async () => {
  if (state === 'running') {
    timeout = setTimeout(() => {
      requestAnimationFrame(renderLoop);
      render();
    }, getAnimationInterval());
  } else {
    clearTimeout(timeout);
    timeout = null;
  }
};

const pause = () => {
  state = 'paused';
  toggle.innerText = "▶";
  clearTimeout(timeout);
  timeout = null;
};

const start = () => {
  state = 'running';
  toggle.innerText = "⏸";
  if (timeout === null) {
    requestAnimationFrame(renderLoop);
  }
};

toggle.onclick = () => {
  if (state === 'running') {
    pause();
  } else {
    start();
  }
};

step.onclick = () => {
  if (state === 'running') {
    pause();
  } else {
    requestAnimationFrame(render);
  }
}

start();

clear.onclick = () => {
  universe = instantiateFromUniverse(wasm.create_empty_universe());
  drawCells();
}

randomise.onclick = () => {
  universe = instantiateFromUniverse(wasm.create_random_universe());
  drawCells();
}
