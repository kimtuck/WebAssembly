const js = import("./app_wasm");
js.then(js => {
    js.greet("World!");
});