const js = import("./node_modules/@raindust/hello-wasm/hello_wasm.js");
js.then(js => {
  js.greet("WebAssembly");
});
