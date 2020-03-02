// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import wasmInit, { hello, run } from "../pkg/wasm_svg_examples_triangle.js";

const wasmRun = async () => {
  const r = await wasmInit("../pkg/wasm_svg_examples_triangle_bg.wasm");

  hello("Triangle Example");

  run();

};
wasmRun();
