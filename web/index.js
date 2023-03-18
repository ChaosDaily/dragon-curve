import { memory } from "dragon-curve/dragon_curve_bg";
import { Universe } from "dragon-curve";

const CELL_LEN = 5;
const COLOR = "#000000";
const HEIGHT = 1920;
const WIDTH = 1080;
const START_X = 500;
const START_Y = 1000;

var last_x = START_X;
var last_y = START_Y;

// Create a new universe.
var universe = Universe.new(CELL_LEN, START_X, START_Y);

// Get elements from the DOM.
const canvas = document.getElementById("dragon-curve-canvas");
canvas.height = HEIGHT;
canvas.width = WIDTH;


const ctx = canvas.getContext('2d');

// The render loop.
const renderLoop = () => {
  draw();
  universe.tick();
  requestAnimationFrame(renderLoop);
};

// Draw each framee
const draw = () => {
  const pointsPtr = universe.points();
  const points = new Float64Array(memory.buffer, pointsPtr, 2);

  ctx.beginPath();
  ctx.strokeStyle = COLOR;

  // Draw line from point to point
  let x = points[0];
  let y = points[1];

  ctx.moveTo(last_x, last_y);
  ctx.lineTo(x, y);
  last_x = x;
  last_y = y;

  ctx.stroke();
};

requestAnimationFrame(renderLoop);
// for (let i = 0; i < 1000; i += 1) {
//   draw();
//   universe.tick();
// }
