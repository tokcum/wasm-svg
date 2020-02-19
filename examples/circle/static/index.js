// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import wasmInit, { hello, run } from "../pkg/wasm_svg_examples_circle.js";

function Sleep(milliseconds) {
   return new Promise(resolve => setTimeout(resolve, milliseconds));
}

const wasmRun = async () => {
  const r = await wasmInit("../pkg/wasm_svg_examples_circle_bg.wasm");

  run();

};
wasmRun();
