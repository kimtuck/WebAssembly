
function update(s,count) {
    $('#elapsed').text(s);
}
module.exports = update;

const js = import("./app_wasm");
js.then(js => {
    debugger

    var letters = () => "abcdefghijklmnopqrstuvwxyz".split("").reduce((accum, letter) => { accum[letter] = 0; return accum; },{} );

    var letterReducer = (counts, letter) => {
        counts[letter]=1;
        return counts;
    }

    var wordReducer = (counts, word) => {
        var wordCounts = word.split("").reduce(letterReducer, letters());
        var result = Object.keys(counts).reduce((accum, key) => { accum[key] = counts[key] + wordCounts[key]; return accum },{} )
        return result;
    }

    var letterCounts = (str) => {
        var words = str.split(" ");
        var wordsLetterCount = words.reduce(wordReducer, letters());
        return wordsLetterCount;
        //return Promise.resolve(wordsLetterCount)
    }

    //console.log('xxx', wordReducer(letters(), "abcf"));
    //console.log(letterCounts("abcf bcf ag"));

    fetch('./MobyDick.txt')
        .then(response => response.text())
        .then(text => {
            /*
            var a = performance.now()
            var counts = letterCounts(text);
            var b = performance.now();
            showResults(counts);
            showElapsed(b-a);
            */

            js.count_letters_in_words(text)


        })
});

