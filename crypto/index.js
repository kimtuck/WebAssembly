
import

{ letterCount_js } from "./lettercount";

const js = import("./app_wasm");
js.then(js => {
    fetch('./MobyDick.txt')
        .then(response => response.text())
        .then(text => text.toLowerCase().replace(/[^a-z ]/g,''))
        .then(text => {
            // Web assembly
            var a = performance.now()
            js.letter_count_webassembly(text)
            var b = performance.now();
            document.getElementById('elapsed-wasm').innerHTML = (b - a);
            return text;
        })
        .then(text => {

            // Javascript
            var a = performance.now()
            var list = letterCount_js(text);
            //display_js(list);
            var b = performance.now();
            document.getElementById('elapsed-js').innerHTML = (b-a);
            return text;
        })
});

var display_js = function(vals) {
    var arr = Object.keys(vals).map(function(key) {
        return { key: key, count: vals[key] };
    });

    var root = document.getElementById("results-js");
    var ul = document.createElement("ul");
    root.appendChild(ul);
    arr.forEach(x => {
        var li = document.createElement('li');
        var letter = document.createElement('span');
        letter.innerHTML = x.key;
        var val = document.createElement('span');
        val.innerHTML = x.count;
        li.appendChild(letter);
        li.appendChild(val);
        ul.appendChild(li);
    });
}