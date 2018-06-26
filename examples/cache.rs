//! A sample application asynchronously printing metrics to stdout.

extern crate dipstick;

use std::thread::sleep;
use std::time::Duration;
use std::io;
use dipstick::*;

fn main() {
    let metrics = Text::output(io::stdout()).with_metric_cache(5).open_scope().add_prefix("cache");

    loop {
        // report some ad-hoc metric values from our "application" loop
        metrics.counter("blorf").count(1134);
        metrics.marker("burg").mark();

        sleep(Duration::from_millis(500));
    }
}
