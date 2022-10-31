use graph::matrix_graph::MatrixGraph;
use graph::serialization;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let tgf_path = get_tgf_path().expect("Pass path to TGF file");
    let tgf = load_tgf_as_str(&tgf_path);

    let g: MatrixGraph<String, String> =
        serialization::de_tgf(&tgf).expect("Failed to deserialize graph");

    println!("Result graph:\n{}", g);
}

fn get_tgf_path() -> Option<String> {
    env::args().nth(1)
}

fn load_tgf_as_str(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    contents
}
