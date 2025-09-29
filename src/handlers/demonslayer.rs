use actix_files::NamedFile;
use actix_web::{get, web, Error, error};
use std::env;
use crate::utils::pathgenerator::generate_path;

#[get("/demonslayer")]
async fn get_demonslayer_image() -> Result<NamedFile, Error> {
    let demonslayer_path = env::var("DEMONSLAYER").expect("DEMONSLAYER path should be set in .env file");
    let result = generate_path(&demonslayer_path);
    match result {
        Some(path) => Ok(NamedFile::open(path)?),
        None => Err(error::ErrorNotFound("No photos found"))
    }
}

pub fn demonslayer_images(cfg: &mut web::ServiceConfig) {
    cfg.service(get_demonslayer_image);
}