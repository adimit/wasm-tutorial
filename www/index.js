import * as wasm from "wasm-gol";

const pre = document.getElementById("game-of-life-canvas");
const universe = wasm.create_universe();
console.log(universe);

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}
const renderLoop = async () => {
  pre.textContent = universe.render_as_string();

  await sleep(100);
  universe.tick();
  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
