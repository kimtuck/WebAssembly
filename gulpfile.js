gulp = require('gulp');
var exec = require('child_process').exec;

gulp.task('build', function (cb) {
    exec('cargo build --target=wasm32-unknown-emscripten --release', function (err, stdout, stderr) {
        console.log(stdout);
        console.log(stderr);
        cb(err);
    });
});

gulp.task('serve', function (cb) {
    exec('python -m SimpleHTTPServer', function (err, stdout, stderr) {
        console.log(stdout);
        console.log(stderr);
        cb(err);
    });
});

gulp.task('copy', function() {
    gulp.src('./target/wasm32-unknown-emscripten/release/deps/*.wasm')
        .pipe(gulp.dest('./site'));
    gulp.src('./target/wasm32-unknown-emscripten/release/deps/*.js')
        .pipe(gulp.dest('./site'));
});