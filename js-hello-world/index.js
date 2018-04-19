const js = import("./js_hello_world");

js.then(js => {
    js.greet("World!");
});