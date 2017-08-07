extern crate includedir_codegen;

use includedir_codegen::Compression;

fn main() {
    includedir_codegen::start("WWW")
        .dir("data/www", Compression::Gzip)
         .build("data_www.rs")
        .unwrap();
}