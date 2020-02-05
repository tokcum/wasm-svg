// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import wasmInit, { hello, run, nameit, append_svg, append_rect, move_rect } from "../pkg/wasm_svg_examples_simple.js";

function Sleep(milliseconds) {
   return new Promise(resolve => setTimeout(resolve, milliseconds));
}

const wasmRun = async () => {
  const r = await wasmInit("../pkg/wasm_svg_examples_simple_bg.wasm");

  run();

  nameit("#svgcontainer");

  append_svg("#svgcontainer", "svg")

  await Sleep(3000);

  append_rect("svg", "svg");

  await Sleep(3000);

  append_rect("svg", "svg");

  await Sleep(3000);

  move_rect("rect", "svg");
};
wasmRun();
