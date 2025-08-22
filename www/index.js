
import { Universe } from "../pkg/wasm_game_of_life";

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new();

const renderLoop = () => {
  pre.textContent = universe.render();
  universe.tick();

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);

/*
const width = 2123;
const height = 2123;

// ランダム初期化
let cells = Array.from({ length: width * height }, (_, i) =>
  (i % 11 === 0 || i % 37 === 0) ? 1 : 0
);

function getIndex(x, y) {
  return y * width + x;
}

function liveNeighborCount(x, y) {
  let cnt = 0;
  for (let dy of [-1, 0, 1]) {
    for (let dx of [-1, 0, 1]) {
      if (dx === 0 && dy === 0) continue;
      const nx = (x + dx + width) % width;
      const ny = (y + dy + height) % height;
      if (cells[getIndex(nx, ny)] === 1) cnt++;
    }
  }
  return cnt;
}

function tick() {
  const next = cells.slice();
  for (let i = 0; i < 20; i++) { // 20 iterations
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const idx = getIndex(x, y);
      const cell = cells[idx];
      const liveNeighbors = liveNeighborCount(x, y);

      let nextCell;
      if (cell === 1 && liveNeighbors < 2) nextCell = 0;
      else if (cell === 1 && (liveNeighbors === 2 || liveNeighbors === 3)) nextCell = 1;
      else if (cell === 1 && liveNeighbors > 3) nextCell = 0;
      else if (cell === 0 && liveNeighbors === 3) nextCell = 1;
      else nextCell = cell;

      next[idx] = nextCell;
    }
  }
  }
  cells = next;
}

function render() {
  let out = "";
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      out += cells[getIndex(x, y)] === 1 ? "◼" : "◻";
    }
    out += "\n";
  }
  return out;
}

const pre = document.getElementById("game-of-life-canvas");

function renderLoop() {
  pre.textContent = render();
  tick();
  requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);
*/