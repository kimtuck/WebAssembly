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

var letterCount = (str) => {
    var words = str.split(" ");
    var wordsLetterCount = words.reduce(wordReducer, letters());
    return wordsLetterCount;
    //return Promise.resolve(wordsLetterCount)
}

export { letterCount }