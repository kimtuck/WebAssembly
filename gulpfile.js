gulp = require('gulp');

gulp.task('copy', function() {
    gulp.src('./target/wasm32-unknown-emscripten/release/deps/*.wasm')
        .pipe(gulp.dest('./site'));
    gulp.src('./target/wasm32-unknown-emscripten/release/deps/*.js')
        .pipe(gulp.dest('./site'));
});