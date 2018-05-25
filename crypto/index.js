
import

{ letterCount } from "./lettercount";

const js = import("./app_wasm");
js.then(js => {
    fetch('./MobyDick.txt')
        .then(response => response.text())
        .then(text => {

            text=text.toLowerCase();
            console.log(text);

            var a = performance.now()
            js.count_letters_in_words(text)
            var b = performance.now();
            document.getElementById('elapsed-wasm').innerHTML = (b-a);


            var a = performance.now()
            var list = letterCount(text);
            //display_js(list);
            var b = performance.now();
            document.getElementById('elapsed-js').innerHTML = (b-a);



            /*
            var a = performance.now()
            var counts = letterCounts(text);
            var b = performance.now();
            showResults(counts);
            showElapsed(b-a);
            */

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