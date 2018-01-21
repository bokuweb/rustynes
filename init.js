import start from './main.js';

const name = "wasm/rustynes";
window.Module = {
  preRun: [],
  postRun: [],
  wasmBinaryFile: `${name}.wasm`,
  noExitRuntime: true,
  print: text => {
    console.log(text);
  },
  printErr: text => {
    console.error(text);
  },
  onRuntimeInitialized() {
    start().catch(e => {
      if (e == 'SimulateInfiniteLoop') {
        Module['noExitRuntime'] = true;
      }
    });
  },
};
fetch(`${name}.wasm`)
  .then(resp => resp.arrayBuffer())
  .then(buffer => {
    Module.wasmBinary = buffer;
    const script = document.createElement("script");
    script.src = `${name}.js`;
    document.body.appendChild(script);
  })