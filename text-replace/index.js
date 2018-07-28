const js = import("./app_wasm");
js.then(js => {


    function process() {
        let from = $('#from').val();
        let to = $('#to').val();
        let text = $('#text').val();
        js.text_replace(from, to, text);
    }

    $(function() {
        $("input").keyup(process);
    });
})

function update(s) {
    $('#result').text(s);
}

