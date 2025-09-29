use std::fs;
use std::path::PathBuf;
use rand::prelude::IndexedRandom;
use rand::rng;

pub fn generate_path(path: &str) -> Option<PathBuf>{
    let photos: Vec<_> = fs::read_dir(path)
        .ok()?
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .map(|e| e.path())
        .collect();
    photos.choose(&mut rng()).cloned()
}