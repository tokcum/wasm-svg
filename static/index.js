// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import wasmInit, { hello, run, nameit, append } from "../pkg/wasm_svg_lib.js";

const wasmRun = async () => {
  const r = await wasmInit("../pkg/wasm_svg_lib_bg.wasm");

/*
  const h = hello('Space & World');

  console.log(h);
  document.body.textContent = h;
*/
  run();


  nameit("#svgcontainer");
  append("#svgcontainer", "svg")
};
wasmRun();
