#![feature(test)]

use stream_cipher::bench_sync;
bench_sync!(salsa20::Salsa20);
