use std::{env, fs::OpenOptions, io::Write, path::Path};

use yarnwrap_docgen::Index;

fn main() {
    let mut index = Index::default();
    index.populate(Path::new(&env::args().nth(1).unwrap_or(".".to_string())));

    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(Path::new(
            &env::args().nth(2).unwrap_or("index.json".to_string()),
        ))
        .unwrap()
        .write_all(&serde_json::to_vec(&index).unwrap())
        .unwrap();
}
